import "./synth-panel.scss";
import classNames from "classnames";
import {FC, PropsWithChildren} from "react";

type IProps = {
  caption?: string;

  left: number;
  top: number;
  width: number;
  height: number;

  className?: string;
};

export const SynthPanel: FC<PropsWithChildren<IProps>> = (props) => {

  const {
    left, top, width, height
  } = props;

  return <div className="synth-panel-container" style={{left, top, width, height}}>
    {props.caption && <div className="synth-panel-caption">
      {props.caption}
    </div>}
    <div className={classNames("synth-panel", props.className || "")}>
      {props.children}
    </div>
  </div>
}
