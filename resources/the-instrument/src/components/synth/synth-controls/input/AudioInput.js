import Tooltip from '~/Tooltip/Tooltip';
import PatchPoint from '~/Controls/PatchPoint/PatchPoint';

export default function AudioInput({
  module,
  control,
  position,
  children,
  uuid,
  modulePosition,
  channel,
}) {
  const inputLabel = module.getInputsConfig()[control]?.label;

  return (
    <Tooltip content={inputLabel || uuid}>
      <PatchPoint position={position} type="audio-input" controlUuid={uuid} modulePosition={modulePosition} module={module} control={control} channel={channel}>
        {children}
      </PatchPoint>
    </Tooltip>
  );
}
