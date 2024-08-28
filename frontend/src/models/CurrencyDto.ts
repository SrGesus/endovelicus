export default class CurrencyDto {
  code?: string
  name?: string
  symbol?: string
  rate?: number

  constructor (jsonObj: Partial<CurrencyDto>) {
    Object.assign(this, jsonObj)
  }
}
