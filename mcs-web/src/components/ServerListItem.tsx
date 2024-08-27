// import './ServerListItem.css';

export default function ServerListItem(props: {
  id: string,
  online: boolean,
  display_name: string,
  description: string,
  players: {
    active: number,
    total: number
  },
  software: string,
  modpack: string | null,
}) {
  return (
    <div class='bg-dark-server-background mb-3 p-2 rounded-md'>
      <a href={`/server/${props.id}/`} class="grid grid-cols-2">
        <p class='text-xl ml-3'>{props.display_name}</p>
        <p class='ml-6 text-dark-server-description col-start-1'>{props.description}</p>
        {
          props.online ?
            (
              <p class="col-start-2 row-start-1 text-right">Status:
                <span class="text-server-online mx-1">●</span>
                <span>Online</span>
              </p>
            ) :
            (
              <p class="col-start-2 row-start-1 text-right">Status:
                <span class="text-server-offline mx-1">●</span>
                <span>Offline</span>
              </p>
            )
        }
        <p class="text-right mr-3">Players: {props.players.active} / {props.players.total}</p>
        <p class="col-start-2 text-right mr-3">Server: {props.software}</p>
      </a>
    </div>
  )
}