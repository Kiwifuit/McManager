import { AiFillFileText } from "solid-icons/ai";
import { BiRegularWorld } from "solid-icons/bi";
import {
  FaSolidGears,
  FaSolidPuzzlePiece,
  FaSolidServer,
  FaSolidTerminal,
  FaSolidUsers,
} from "solid-icons/fa";
import { FiSliders } from "solid-icons/fi";

import { JSX } from "solid-js/h/jsx-runtime";

export default function DashboardNavBar(props: {
  displayName: string;
  id: string;
}) {
  return (
    <aside class="bg-light-accent dark:bg-dark-accent">
      <ul class="mt-2 flex h-[calc(100vh-3.75rem)] flex-col rounded-xl text-left">
        <p class="truncate px-4 py-2 text-2xl font-bold">{props.displayName}</p>
        <NavbarButton url={`/server/${props.id}/`} name="Server">
          <FaSolidServer />
        </NavbarButton>
        <NavbarButton url={`/server/${props.id}/options`} name="Options">
          <FiSliders />
        </NavbarButton>
        <NavbarButton url={`/server/${props.id}/console`} name="Console">
          <FaSolidTerminal />
        </NavbarButton>
        <NavbarButton url={`/server/${props.id}/logs`} name="Logs">
          <AiFillFileText />
        </NavbarButton>
        <NavbarButton url={`/server/${props.id}/players`} name="Players">
          <FaSolidUsers />
        </NavbarButton>
        <NavbarButton url={`/server/${props.id}/worlds`} name="Worlds">
          <BiRegularWorld />
        </NavbarButton>
        <NavbarButton url={`/server/${props.id}/software`} name="Software">
          <FaSolidGears />
        </NavbarButton>
        <NavbarButton url={`/server/${props.id}/modpacks`} name="Mods">
          <FaSolidPuzzlePiece />
        </NavbarButton>
      </ul>
    </aside>
  );
}

function NavbarButton(props: {
  url: string;
  name: string;
  children?: JSX.Element;
}) {
  return (
    <li>
      <a
        class="dark:hover:bg-dark-dashboard-hover hover:bg-light-dashboard-hover block w-full rounded-sm bg-light-accent px-3 py-2 text-xl shadow-dark-bg active:shadow-inner dark:bg-dark-accent"
        href={props.url}
      >
        <span class="flex items-center gap-3">
          {props.children}
          {props.name}
        </span>
      </a>
    </li>
  );
}
