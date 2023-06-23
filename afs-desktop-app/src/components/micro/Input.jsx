import classes from './Input.module.css'

function Input({ value, label, name, placeholder, type, onChange }) {
    return (
        <div className={classes.inputContainer}>
            <input
                class={classes.input}
                name={name}
                type={type}
                placeholder={placeholder}
                onChange={onChange}
                value={value}
                autocomplete="off"
            />

            <label class={classes.label} for={name}>{label}</label>
        </div>
    );
}
export default Input;