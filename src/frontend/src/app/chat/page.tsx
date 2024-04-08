"use client";

import { useGetChatResponse } from "@/commands";
import { useState } from "react";
import Markdown from "react-markdown";
import rehypeHighlight from "rehype-highlight";

export default function ChatPage() {
  const [prompt, setPrompt] = useState("");
  const { trigger, isMutating, error, data: response } = useGetChatResponse();

  return (
    <div>
      <div className="m-4">
        <div className="mb-2">
          <b>Input</b>
        </div>
        <div className="mb-2">
          <textarea
            name="input"
            className="border-2 border-black p-2 w-3/5 h-min min-h-36"
            onChange={(e) => setPrompt(e.target.value)}
          ></textarea>
        </div>
        <button
          className="p-2 border-2"
          onClick={() => trigger({ prompt })}
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
        <div className="w-3/5">
          {isMutating ? (
            <div>loading...</div>
          ) : error ? (
            <div>Encountered error: {error}</div>
          ) : !response ? (
            <></>
          ) : (
            <Markdown rehypePlugins={[rehypeHighlight]}>{response}</Markdown>
          )}
        </div>
      </div>
    </div>
  );
}
