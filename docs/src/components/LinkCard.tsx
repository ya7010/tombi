import type { Component } from "solid-js";
import { createSignal, onMount, Show } from "solid-js";
import { fetchOGP } from "../utils/ogp";
import type { OGPData } from "../utils/ogp";
import { TbLink } from "solid-icons/tb";

interface LinkCardProps {
  url: string;
}

export const LinkCard: Component<LinkCardProps> = (props) => {
  const [ogp, setOgp] = createSignal<OGPData>({
    title: "",
    description: "",
    image: "",
  });
  const [loading, setLoading] = createSignal(true);
  const [error, setError] = createSignal(false);

  onMount(async () => {
    try {
      const data = await fetchOGP(props.url);
      if (data && !data.title && !data.description && !data.image) {
        setError(true);
      } else if (data) {
        setOgp(data);
      } else {
        setError(true);
      }
    } catch {
      setError(true);
    } finally {
      setLoading(false);
    }
  });

  return (
    <div class="w-auto max-w-xl mx-0 my-6 border border-gray-200 rounded-lg p-4 bg-white shadow-sm transition-all hover:translate-y-[-2px] hover:shadow-md">
      <a
        href={props.url}
        class="block text-inherit no-underline"
        target="_blank"
        rel="noopener noreferrer"
      >
        <Show
          when={!loading()}
          fallback={<div class="animate-pulse h-20 bg-gray-200 rounded" />}
        >
          <Show
            when={!error()}
            fallback={
              <div class="flex items-center gap-2">
                <TbLink class="text-xl text-gray-600" />
                <span class="text-gray-800 break-all">{props.url}</span>
              </div>
            }
          >
            <Show when={ogp() !== null}>
              <div class="flex items-center gap-6 sm:(flex-col text-center gap-2)">
                <img
                  src={ogp().image}
                  alt={ogp().title}
                  class="w-20 h-20 rounded"
                  style="object-fit: cover"
                />
                <div>
                  <h3 class="text-gray-800 text-xl m-0 mb-2">{ogp()?.title}</h3>
                  <p class="text-gray-600 text-sm m-0 hidden sm:block">
                    {ogp()?.description}
                  </p>
                </div>
              </div>
            </Show>
          </Show>
        </Show>
      </a>
    </div>
  );
};
