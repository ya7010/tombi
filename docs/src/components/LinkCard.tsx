import type { Component } from "solid-js";
import type { OgpId } from "../utils/ogp";
import ogpData from "../../ogp.json";

interface LinkCardProps {
  id: OgpId;
}

export const LinkCard: Component<LinkCardProps> = (props) => {
  const data = ogpData[props.id];

  return (
    <div class="w-auto max-w-xl mx-0 my-6 border border-gray-200 rounded-lg p-4 bg-white shadow-sm transition-all hover:translate-y-[-2px] hover:shadow-md">
      <a
        href={data.url}
        class="block text-inherit no-underline"
        target="_blank"
        rel="noopener noreferrer"
      >
        <div class="flex items-center gap-6 sm:(flex-col text-center gap-2)">
          <img
            src={data.image}
            alt={data.title}
            class="w-20 h-20 rounded"
            style="object-fit: cover"
          />
          <div>
            <h3 class="text-gray-800 text-xl m-0 mb-2">{data.title}</h3>
            <p class="text-gray-600 text-sm m-0 hidden sm:block">
              {data.description}
            </p>
          </div>
        </div>
      </a>
    </div>
  );
};
