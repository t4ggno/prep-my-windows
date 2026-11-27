import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

const remarkPlugins = [remarkGfm];

export function Markdown({ children }: { children: string }) {
  return (
    <div className="markdown-body">
      <ReactMarkdown remarkPlugins={remarkPlugins}>{children}</ReactMarkdown>
    </div>
  );
}
