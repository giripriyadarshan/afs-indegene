import classes from './Input.module.css'

function Input({ value, label, name, placeholder, type, onChange }) {
    return (
        <div className={classes.inputContainer}>
            <input
                className={classes.input}
                name={name}
                type={type}
                placeholder={placeholder}
                onChange={onChange}
                value={value}
            />

            <label className={classes.label} htmlFor={name}>{label}</label>
        </div>
    );
}
export default Input;