export default function DashboardNavBar() {
  return (
    <aside class="bg-server-offline">
      <ul class="mt-2 flex flex-col gap-3 text-center">
        <NavbarButton url="./server" name="Server" />
        <NavbarButton url="./options" name="Options" />
        <NavbarButton url="./console" name="Console" />
        <NavbarButton url="./logs" name="Logs" />
        <NavbarButton url="./players" name="Players" />
        <NavbarButton url="./worlds" name="Worlds" />
        <NavbarButton url="./software" name="Software" />
        <NavbarButton url="./modpacks" name="Modpacks" />
      </ul>
    </aside>
  );
}

function NavbarButton(props: { url: string; name: string }) {
  return (
    <li>
      <a
        class="block w-full rounded-sm bg-server-online px-3 py-2 shadow-dark-bg active:shadow-inner"
        href={props.url}
      >
        {props.name}
      </a>
    </li>
  );
}
