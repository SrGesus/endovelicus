import CurrencyDto from '@/models/CurrencyDto'
import axios from 'axios'

const httpClient = axios.create()
httpClient.defaults.timeout = 10000
// httpClient.defaults.baseURL = import.meta.env.VITE_ROOT_API
httpClient.defaults.baseURL = 'http://localhost:3030/api'
httpClient.defaults.headers.post['Content-Type'] = 'application/json'

export default class RemoteService {
  // --------------------- Currency ---------------------
  static async getAllCurrencies (): Promise<CurrencyDto[]> {
    return httpClient.get('/currency').then(response => {
      return (response.data as any[]).map(currency => {
        return new CurrencyDto(currency)
      })
    })
  }

  static async createCurrency (currency: CurrencyDto): Promise<CurrencyDto> {
    return httpClient.post('/currency', currency).then(response => {
      return new CurrencyDto(response.data)
    })
  }

  static async updateCurrency (currency: CurrencyDto): Promise<CurrencyDto> {
    return httpClient.patch('/currency', currency).then(response => {
      return new CurrencyDto(response.data)
    })
  }

  static async deleteCurrency (currency: CurrencyDto): Promise<void> {
    return httpClient.delete('/currency', { params: { code: currency.code } })
  }
}
