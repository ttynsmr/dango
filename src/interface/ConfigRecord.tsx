import { Input } from "@material-tailwind/react";

type Props = {
  configName: string
}

const ConfigRecord: React.FC<Props> = ({ configName }) => {
  return (
    <>
      <Input className="bg-indigo-500" label={configName} />
    </>
  )
};

export default ConfigRecord
