import classes from './LeftNav.module.css';

export default function LeftNav(props) {
    return (
        <section className={classes.leftnav}>
            {props.children}
        </section>
    )
}