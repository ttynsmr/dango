import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Button } from "@material-tailwind/react";
import ConfigRecord from "./ConfigRecord";

type Props = {
  groupName: string,
  configs: string[],
}

const ConfigRecordGroup: React.FC<Props> = ({ groupName, configs }) => {
  return (
    <>
      <div className='w-full flex-row gap-1 p-4 border-purple-600 border-4'>
        <div className="text-purple-50">{groupName}</div>
        {configs.map((value, index, config) => <ConfigRecord key={value} configName={value} />)}
        <Button className="p-1" color="amber"><FontAwesomeIcon className="p-1" icon={["fas", "floppy-disk"]} />Save</Button>
      </div>
    </>
  )
};

export default ConfigRecordGroup
