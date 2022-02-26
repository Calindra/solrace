import { toast } from 'react-toastify'
import { AxiosError } from 'axios'

export const toastAPIError = (e: Error | AxiosError) => {
  let message = 'Something went wrong. Please try again.'

  if ((e as any).isAxiosError) {
    const axiosError = e as AxiosError
    message =
      axiosError.response?.data?.message ||
      axiosError.response?.statusText ||
      message
  }

  toast(message, { type: 'error' })
}