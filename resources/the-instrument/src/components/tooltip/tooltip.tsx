import {FC, PropsWithChildren, useState} from "react";
import "./tooltip.scss";
import {createPortal} from "react-dom";
import {usePopper} from "react-popper";
import {Placement} from "@popperjs/core";
import classNames from "classnames";

type IProps = {
    show: boolean;
    referenceElement: HTMLElement;
    placement?: Placement;
    className?: string;
};

export const Tooltip: FC<PropsWithChildren<IProps>> = ({
                                                           show,
                                                           referenceElement,
                                                           placement,
                                                           className,
                                                           children
                                                       }) => {
    const [popperElement, setPopperElement] = useState<HTMLDivElement | null>(null);
    const [arrowElement] = useState<HTMLDivElement | null>(null);

    const {styles, attributes} = usePopper(referenceElement, popperElement, {
        placement: placement || 'right',
        modifiers: [
            {
                name: 'arrow',
                options: {
                    element: arrowElement,
                }
            },
            {
                name: 'offset',
                options: {
                    offset: [0, 10],
                },
            },
        ],
    });

    return show ?
        createPortal(<div className={classNames("tooltip", {[className || '']: true})} ref={setPopperElement}
                          style={styles.popper} {...attributes.popper}>
            {children}
        </div>, document.querySelector('#tooltip-portal') as Element)
        : null
}
