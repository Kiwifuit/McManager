import './ServerListItem.css';

export default function ServerListItem(props: { id: string, display_name: string, description: string }) {
  return (
    <a href={`/server/${props.id}/`}>
      <div>
        <p>{props.display_name}</p>
        <p>{props.description}</p>
      </div>
    </a>
  )
}