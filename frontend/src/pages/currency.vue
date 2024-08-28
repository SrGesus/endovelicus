<template>
  <v-container class="fill-height">
    <v-form ref="form" style="width: 100%;" validate-on="submit" @submit.prevent="saveCurrency">
      <v-data-table
        class="bg-background border-primary border "
        :headers="headers"
        :items="currencies"
        :v-model:sort-by="{ key: 'name', order: 'asc' }"
      >
        <template #header.actions>
          <v-btn
            class="w-full h-full"
            color="primary"
            prepend-icon="mdi-plus"
            rules="required"
            text="New"
            @click="newCurrency"
          />
        </template>
        <template #item.name="{ index }">
          <v-text-field
            v-if="index === currentRow"
            v-model="currencies[index].name"
            density="compact"
            hide-details="auto"
            :rules="nameRules"
          />
          <div v-else>{{ currencies[index].name }}</div>
        </template>
        <template #item.code="{ index }">
          <v-text-field
            v-if="index === currentRow && currencies[index].new"
            v-model="currencies[index].code"
            density="compact"
            hide-details="auto"
            :rules="codeRules"
          />
          <div v-else>{{ currencies[index].code }}</div>
        </template>
        <template #item.symbol="{ index }">
          <v-text-field
            v-if="index === currentRow"
            v-model="currencies[index].symbol"
            density="compact"
            hide-details="auto"
          />
          <div v-else>{{ currencies[index].symbol || '-' }}</div>
        </template>
        <template #item.rate="{ index }">
          <v-number-input
            v-if="index === currentRow"
            v-model="currencies[index].rate"
            control-variant="stacked"
            density="compact"
            hide-details="auto"
            :rules="rateRules"
            :step="0.01"
          />
          <div v-else>{{ currencies[index].rate }}</div>
        </template>
        <template #item.actions="{ index }">
          <template v-if="index !== currentRow">
            <v-icon
              v-if="index !== currentRow"
              class="mr-2"
              color="primary"
              @click="setCurrent(index)"
            >mdi-pencil</v-icon>
            <v-icon
              v-if="index !== currentRow"
              color="error"
              @click="deleteCurrency(index)"
            >mdi-delete</v-icon>
          </template>
          <template v-else>
            <v-btn
              class="mr-2"
              color="primary"
              @click="saveCurrency"
            >Save</v-btn>
            <v-btn
              variant="flat"
              @click="getCurrencies"
            >Cancel</v-btn>
          </template>
        </template>
      </v-data-table>
    </v-form>
  </v-container>
</template>

<script lang="ts" setup>
  import CurrencyDto from '@/models/CurrencyDto'
  import RemoteService from '@/services/RemoteService'
  import { VNumberInput } from 'vuetify/labs/components'
  import { ref } from 'vue'
  import { VForm } from 'vuetify/components'

  type Currency = CurrencyDto & { new?: boolean }

  const headers = [
    { title: 'Name', value: 'name', key: 'name' },
    { title: 'Code', value: 'code', key: 'code' },
    { title: 'Symbol', value: 'symbol', key: 'symbol' },
    { title: 'Rate', value: 'rate', key: 'rate' },
    { key: 'actions' },
  ]

  const currencies = ref([] as Currency[])

  const form: Ref<null | VForm> = ref(null)

  const currentRow = ref(-1)

  const getCurrencies = async () => {
    currentRow.value = -1
    RemoteService.getAllCurrencies().then(data => {
      currencies.value = data
      console.log(data)
    })
  }
  getCurrencies()

  const deleteCurrency = (i: number) => {
    // Is not new because if there is a new currency, the delete button is not shown
    RemoteService.deleteCurrency(currencies.value[i]).then(() => {
      getCurrencies()
    })
  }

  const newCurrency = () => {
    if (currentRow.value === 0 && currencies.value[currentRow.value].new) {
      return
    }
    currencies.value.unshift({ name: '', code: '', symbol: '', rate: 0, new: true })
    currentRow.value = 0
  }

  const setCurrent = (i: number) => {
    if (currentRow.value !== -1) {
      saveCurrency().then(() => {
        currentRow.value = i
      })
    } else {
      currentRow.value = i
    }
  }

  const saveCurrency = async () => {
    return form.value!.validate().then(({ valid }) => {
      if (valid) {
        if (currencies.value[currentRow.value].new) {
          return RemoteService.createCurrency(currencies.value[currentRow.value]).then(() => {
            getCurrencies()
          })
        } else {
          return RemoteService.updateCurrency(currencies.value[currentRow.value]).then(() => {
            getCurrencies()
          })
        }
      } else {
        throw new Error('Validation error')
      }
    })
  }

  const codeRules = [
    (v: string) => v ? true : 'Code is required',
    (v: string) => /[a-zA-Z]{3}/.test(v) ? true : 'Code must be 3 capital letters',
    (v: string) => currencies.value.every((c, i) => c.code !== v || i === currentRow.value) ? true : 'Code must be unique',
  ]

  const nameRules = [
    (v: string) => v ? true : 'Name is required',
  ]

  const rateRules = [
    (v: number) => v > 0 ? true : 'Rate must be greater than 0',
  ]

</script>
