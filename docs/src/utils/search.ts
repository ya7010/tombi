import FlexSearch from "flexsearch";
import searchIndex from "../search-index.json";

export interface SearchResult {
  title: string;
  content: string;
  url: string;
  score: number;
}

interface DocumentIndex {
  id: number;
  title: string;
  content: string;
  url: string;
  score?: number;
}

// Create FlexSearch instance
const index = new FlexSearch.Document<DocumentIndex, true>({
  document: {
    id: "id",
    index: ["title", "content"] as const,
    store: true,
  },
  tokenize: "forward",
  context: true,
});

// Add indexes when initialized
for (const doc of searchIndex) {
  index.add(doc as DocumentIndex);
}

export async function searchDocumentation(
  query: string,
): Promise<SearchResult[]> {
  if (!query.trim()) {
    return [];
  }

  try {
    // Search both title and content
    const titleResults = await index.searchAsync(query, {
      index: "title",
      limit: 5,
      enrich: true,
    });
    const contentResults = await index.searchAsync(query, {
      index: "content",
      limit: 5,
      enrich: true,
    });

    // Merge results and remove duplicates
    const resultMap = new Map<number, SearchResult>();

    // Calculate scores and merge
    for (const result of [...titleResults, ...contentResults]) {
      if (result.result) {
        for (const docId of result.result) {
          const doc = searchIndex.find((d) => d.id === Number(docId));
          if (doc && !resultMap.has(doc.id)) {
            resultMap.set(doc.id, {
              title: doc.title,
              content: doc.content,
              url: doc.url,
              score: docId.doc.score || 0,
            });
          }
        }
      }
    }

    // Sort by score and return
    return Array.from(resultMap.values()).sort((a, b) => b.score - a.score);
  } catch (error) {
    console.error("Error during search:", error);
    return [];
  }
}
