import "../../css/components/icons/CheckBox.css"

export function CheckedCheckBoxIcon() {
    return (
        <svg
            className='task-status-done-icon'
            width='26'
            height='26'
            viewBox='0 0 26 26'
            fill='none'
            xmlns='http://www.w3.org/2000/svg'
        >
            <path
                fill='currentColor'
                d='M18.4167 20.5833H7.58332C6.38671 20.5833 5.41666 19.6133 5.41666 18.4167V7.58332C5.41666 6.38671 6.38671 5.41666 7.58332 5.41666H18.4167C19.6133 5.41666 20.5833 6.38671 20.5833 7.58332V18.4167C20.5833 19.6133 19.6133 20.5833 18.4167 20.5833ZM7.58332 7.58332V18.4167H18.4167V7.58332H7.58332ZM11.9167 16.6422L8.99166 13.7746L10.5083 12.2254L11.9167 13.6023L15.4917 10.0631L17.0083 11.6036L11.9167 16.6411V16.6422Z'
            />
        </svg>
    );
}

export function UncheckedCheckBoxIcon() {
    return (
        <svg
            className='task-status-todo-icon'
            width='26'
            height='26'
            viewBox='0 0 26 26'
            fill='none'
            xmlns='http://www.w3.org/2000/svg'
        >
            <path
                fill='currentColor'
                d='M18.4167 20.5833H7.58332C6.38671 20.5833 5.41666 19.6133 5.41666 18.4167V7.58332C5.41666 6.38671 6.38671 5.41666 7.58332 5.41666H18.4167C19.6133 5.41666 20.5833 6.38671 20.5833 7.58332V18.4167C20.5833 19.6133 19.6133 20.5833 18.4167 20.5833ZM7.58332 7.58332V18.4167H18.4167V7.58332H7.58332Z'
            />
        </svg>
    );
}
