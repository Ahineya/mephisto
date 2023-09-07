import Tooltip from '~/Tooltip/Tooltip';
import PatchPoint from '~/Controls/PatchPoint/PatchPoint';

export default function Output({
  module,
  control,
  position,
  children,
  uuid,
  modulePosition,
}) {
  const outputLabel = module.getOutputsConfig()[control].label;

  return (
    <Tooltip content={outputLabel || uuid}>
      <PatchPoint position={position} type="output" controlUuid={uuid} modulePosition={modulePosition} module={module} control={control}>
        {children}
      </PatchPoint>
    </Tooltip>
  );
}
