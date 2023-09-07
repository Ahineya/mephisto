import Tooltip from '~/Tooltip/Tooltip';
import PatchPoint from '~/Controls/PatchPoint/PatchPoint';

export default function Input({
  module,
  control,
  position,
  children,
  uuid,
  modulePosition,
}) {
  const inputLabel = module.getInputsConfig()[control]?.label;

  return (
    <Tooltip content={inputLabel || uuid}>
      <PatchPoint position={position} type="input" controlUuid={uuid} modulePosition={modulePosition} module={module} control={control}>
        {children}
      </PatchPoint>
    </Tooltip>
  );
}
