export interface OGPData {
  title: string;
  description: string;
  image: string;
}

export async function fetchOGP(url: string): Promise<OGPData | null> {
  try {
    const response = await fetch(
      `https://api.allorigins.win/get?url=${encodeURIComponent(url)}`,
    );
    const { contents } = await response.json();

    // Simple HTML parsing to extract meta tags
    const parser = new DOMParser();
    const doc = parser.parseFromString(contents, "text/html");

    const getMetaContent = (property: string) => {
      const meta = doc.querySelector(`meta[property="${property}"]`);
      return meta?.getAttribute("content") || "";
    };

    return {
      title: getMetaContent("og:title") || doc.title || "",
      description: getMetaContent("og:description") || "",
      image: getMetaContent("og:image") || "",
    };
  } catch (error) {
    console.error("Failed to fetch OGP:", error);
    return null;
  }
}
