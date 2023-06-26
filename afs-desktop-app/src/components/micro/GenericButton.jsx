import classes from './GenericButton.module.css'

export default function GenericButton({ disabled, onClick, label }) {
    return (
        <button 
        className={classes.button}
        disabled={disabled}
        onClick={onClick}
        >
            <p>{label}</p>
        </button>
    )
}