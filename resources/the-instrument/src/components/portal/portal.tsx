import ReactDOM from 'react-dom';
import { FC, PropsWithChildren } from 'react';

export const Portal: FC<PropsWithChildren> = ({ children }) => {
  return ReactDOM.createPortal(children, document.body);
};
