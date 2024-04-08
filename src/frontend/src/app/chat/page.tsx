"use client";

import { useGetChatResponse } from "@/commands";
import { Dispatch, SetStateAction, useCallback, useState } from "react";
import Markdown from "react-markdown";
import rehypeHighlight from "rehype-highlight";
import { Plugin } from "unified";
import { Root as HastRoot, RootContent } from "hast";
import { QueryOutput } from "@/components/QueryOutput";

const addRunButtonToCodeBlocks: Plugin<[], HastRoot, HastRoot> =
  () => (tree) => {
    const newChildren: RootContent[] = [];
    let n = 0;

    for (const child of tree.children) {
      newChildren.push(child);

      if (child.type === "element") {
        if (child.tagName === "pre") {
          child.children.push({
            type: "element",
            tagName: "RunCodeBlockButton",
            properties: { n },
            children: [],
          });

          newChildren.push({
            type: "element",
            tagName: "RunQuery",
            properties: { n },
            children: [],
          });

          n++;
        } else {
          newChildren.push({
            type: "element",
            tagName: "div",
            properties: {},
            children: [],
          });
        }
      }
    }

    tree.children = newChildren;
  };

function RunCodeBlockButton({
  n,
  setShouldRunQueries,
}: {
  n: number;
  setShouldRunQueries: Dispatch<SetStateAction<number[]>>;
}) {
  const onClick = useCallback(() => {
    setShouldRunQueries((prevState) => [...prevState, n]);
  }, []);

  return (
    <button
      className="p-2 border-2 text-2xl absolute top-0 right-0"
      title="Run Query"
      onClick={onClick}
    >
      🏃
    </button>
  );
}

function RunQuery({
  n,
  queries,
  shouldRunQueries,
}: {
  n: number;
  queries: string[];
  shouldRunQueries: number[];
}) {
  if (shouldRunQueries.indexOf(n) !== -1) {
    return (
      <div className="ml-12">
        <QueryOutput query={queries[n]} />
      </div>
    );
  } else {
    return <div></div>;
  }
}

export default function ChatPage() {
  const [prompt, setPrompt] = useState("");
  const { trigger, isMutating, error, data: response } = useGetChatResponse();
  const [shouldRunQueries, setShouldRunQueries] = useState<number[]>([]);

  const queries = response ? extractQueries(response) : [];

  const onSubmit = useCallback(() => {
    setShouldRunQueries([]);
    trigger({ prompt });
  }, [trigger, prompt]);

  return (
    <div>
      <div className="m-4">
        <div className="mb-2">
          <b>Input</b>
        </div>
        <div className="mb-2">
          <textarea
            name="input"
            className="border-2 border-black p-2 w-1/2 h-min min-h-36"
            onChange={(e) => setPrompt(e.target.value)}
          ></textarea>
        </div>
        <button
          className="p-2 border-2"
          onClick={onSubmit}
          disabled={isMutating}
        >
          Submit
        </button>
      </div>
      <div className="border-b-2"></div>
      <div className="m-4">
        <div className="mb-2">
          <b>Response</b>
        </div>
        <div>
          {isMutating ? (
            <div>loading...</div>
          ) : error ? (
            <div>Encountered error: {error}</div>
          ) : !response ? (
            <></>
          ) : (
            <div className="space-y-4 grid grid-cols-2">
              <Markdown
                rehypePlugins={[rehypeHighlight, addRunButtonToCodeBlocks]}
                components={{
                  // @ts-ignore
                  RunCodeBlockButton: (props) => {
                    return (
                      <RunCodeBlockButton
                        {...props}
                        setShouldRunQueries={setShouldRunQueries}
                      />
                    );
                  },
                  // @ts-ignore
                  RunQuery: (props) => {
                    return (
                      <RunQuery
                        {...props}
                        queries={queries}
                        shouldRunQueries={shouldRunQueries}
                      />
                    );
                  },
                  pre: (props) => {
                    return <pre {...props} className="relative" />;
                  },
                }}
              >
                {response}
              </Markdown>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
function extractQueries(markdownString: string): string[] {
  const codeBlocksRegex = /```(?:\w+)?\s*([\s\S]+?)\s*```/g;
  const codeBlocks = [];
  let match;

  while ((match = codeBlocksRegex.exec(markdownString)) !== null) {
    codeBlocks.push(match[1]);
  }

  return codeBlocks;
}
