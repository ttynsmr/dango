import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Button, Card } from "@material-tailwind/react";
import Config from "../models/Config";
import ConfigRecord from "./ConfigRecord";

type Props = {
  groupName: string,
  configs: Config[],
}

const ConfigRecordGroup: React.FC<Props> = ({ groupName, configs }) => {
  return (
    <>
      <div className='w-full flex-row gap-1 p-4 border-purple-600 border-4 grid grid-cols-1 place-items-stretch'>
        <div className="text-purple-50">{groupName}</div>
        <Card className="p-2 gap-2">
          {configs.map((value) => <ConfigRecord key={`${value.service}-${value.key}`} configName={value.keyName} />)}
          <Button className="place-self-end" ><FontAwesomeIcon className="pr-1" icon={["fas", "floppy-disk"]} />Save</Button>
        </Card>
      </div>
    </>
  )
};

export default ConfigRecordGroup
