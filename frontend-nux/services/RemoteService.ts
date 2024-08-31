import axios from 'axios'
import CurrencyDto from '@/models/CurrencyDto'

const httpClient = axios.create()
httpClient.defaults.timeout = 10000
// httpClient.defaults.baseURL = import.meta.env.VITE_ROOT_API
httpClient.defaults.baseURL = 'http://localhost:3030/api'
httpClient.defaults.headers.post['Content-Type'] = 'application/json'

// --------------------- Currency ---------------------
export async function getAllCurrencies(): Promise<CurrencyDto[]> {
  return httpClient.get('/currency').then((response) => {
    return (response.data as CurrencyDto[]).map((currency) => {
      return new CurrencyDto(currency)
    })
  })
}

export async function createCurrency(currency: CurrencyDto): Promise<CurrencyDto> {
  return httpClient.post('/currency', currency).then((response) => {
    return new CurrencyDto(response.data)
  })
}

export async function updateCurrency(currency: CurrencyDto): Promise<CurrencyDto> {
  return httpClient.patch('/currency', currency).then((response) => {
    return new CurrencyDto(response.data)
  })
}

export async function deleteCurrency(currency: CurrencyDto): Promise<void> {
  return httpClient.delete('/currency', { params: { code: currency.code } })
}

export default { getAllCurrencies, createCurrency, updateCurrency, deleteCurrency }
