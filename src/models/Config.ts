class Config {
  constructor(service: string, key: string, keyName: string, value: string) {
    this.service = service
    this.key = key
    this.keyName = keyName
    this.value = value
  }
  service: string
  key: string
  keyName: string
  value: string
}

export default Config
