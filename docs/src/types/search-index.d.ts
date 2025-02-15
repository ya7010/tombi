interface SearchIndexItem {
  id: number;
  title: string;
  content: string;
  url: string;
}

declare module "*/search-index.json" {
  const value: SearchIndexItem[];
  export default value;
}
