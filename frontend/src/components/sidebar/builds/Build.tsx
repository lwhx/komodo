import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses } from "../../../util/helpers";
import s from "../sidebar.module.css";

const Build: Component<{ id: string }> = (p) => {
  const { builds, selected } = useAppState();
  const build = () => builds.get(p.id)!;
  return (
    <Show when={build()}>
      <button
        class={combineClasses(
          s.DropdownItem,
          selected.id() === p.id && "selected"
        )}
        onClick={() => selected.set(build()._id!, "build")}
      >
        <div>{build().name}</div>
      </button>
    </Show>
  );
};

export default Build;
