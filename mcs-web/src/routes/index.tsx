import { Title } from "@solidjs/meta";
import ServerListItem from "~/components/ServerListItem";

export default function Home() {
  const props = {
    id: "server1",
    display_name: "Display name for test",
    description: "test description"
  };

  return (
    <main class="grid place-items-center w-screen mt-32">
      <Title>Servers</Title>
      <div class="w-1/2">
        <h1 class="text-4xl font-bold mb-5">Servers:</h1>
        <ServerListItem
          id="server-1"
          online={true}
          display_name="Server 01 Name"
          description="Server 01 Description"
          players={{ total: 20, active: 15 }}
          modpack="test"
          software="Forge 1.2.3 for Minecraft 1.20.1" />
        <ServerListItem
          id="server-2"
          online={true}
          display_name="Server 02 Name"
          description="Server 02 Description"
          players={{ total: 20, active: 15 }}
          modpack="test2"
          software="Forge 1.2.3 for Minecraft 1.20.1" />
        <ServerListItem
          id="server-3"
          online={true}
          display_name="Server 03 Name"
          description="Server 03 Description"
          players={{ total: 20, active: 10 }}
          modpack="test3"
          software="Neoforge 1.2.3 for Minecraft 1.20.1" />
        <ServerListItem
          id="server-4"
          online={false}
          display_name="Server 04 Name"
          description="Server 04 Description"
          players={{ total: 20, active: 0 }}
          modpack={null}
          software="Quilt 1.2.3 for Minecraft 1.20.1" />
        <ServerListItem
          id="server-5"
          online={true}
          display_name="Server 05 Name"
          description="Server 05 Description"
          players={{ total: 20, active: 20 }}
          modpack={null}
          software="Fabric 1.2.3 for Minecraft 1.20.1" />
      </div>
    </main>
  );
}
