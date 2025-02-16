import { For } from "solid-js";
import { A } from "@solidjs/router";
import styles from "./Sidebar.module.css";

type MenuItem = {
  title: string;
  path: string;
  children?: MenuItem[];
};

const menuItems: MenuItem[] = [
  {
    title: "Overview",
    path: "/documentation",
  },
  {
    title: "Concept",
    path: "/documentation/concept",
    children: [
      {
        title: "Overview",
        path: "/concept/overview",
      },
    ],
  },
  {
    title: "Formatter",
    path: "/documentation/formatter",
    children: [
      {
        title: "Magic Trailing Comma",
        path: "/documentation/formatter/magic-trailing-comma",
      },
    ],
  },
];

const TreeItem = (props: { item: MenuItem; level: number }) => {
  return (
    <div
      class={styles.treeItem}
      style={{ "padding-left": `${props.level * 1}rem` }}
    >
      <A href={props.item.path} class={styles.link}>
        {props.item.title}
      </A>
      {props.item.children && (
        <div class={styles.children}>
          <For each={props.item.children}>
            {(child) => <TreeItem item={child} level={props.level + 1} />}
          </For>
        </div>
      )}
    </div>
  );
};

export function Sidebar() {
  return (
    <nav class={styles.sidebar}>
      <For each={menuItems}>{(item) => <TreeItem item={item} level={0} />}</For>
    </nav>
  );
}
