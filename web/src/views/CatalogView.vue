<template>
  <div>
    <div style="height: 600px; width: 700px">
      <l-map
        ref="map"
        v-model:zoom="zoom"
        :center="center"
        :use-global-leaflet="false"
        @ready="onMapReady"
      >
        <l-tile-layer
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          layer-type="base"
          name="OpenStreetMap"
        ></l-tile-layer>

        <l-geo-json v-if="geojsonFeatures" :geojson="geojsonFeatures" :options="geoJsonOptions" />
      </l-map>
    </div>

    <div class="mt-4 flex gap-2">
      <div>
        <label
          for="zip-upload"
          class="inline-block bg-green-600 text-white px-4 py-2 rounded cursor-pointer hover:bg-blue-700"
        >
          Upload ZIP File
        </label>
        <input id="zip-upload" type="file" class="hidden" @change="handleUpload" accept=".zip" />
      </div>

      <button
        @click="handleSave"
        :disabled="!file"
        class="bg-green-600 text-white px-4 py-2 rounded cursor-pointer hover:bg-blue-700"
      >
        Save
      </button>
    </div>

    <div v-if="schema.length > 0" class="mt-4 overflow-auto">
      <h3 class="text-lg font-bold mb-2">Preview</h3>
      <table class="min-w-[400px] border border-gray-300 text-sm">
        <thead class="bg-gray-200">
          <tr>
            <th v-for="key in schema" :key="key" class="px-4 py-2 border border-gray-300 text-left">
              {{ key }}
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(feature, index) in geojsonFeatures?.slice(0, 5)"
            :key="index"
            class="hover:bg-gray-50"
          >
            <td v-for="key in schema" :key="key" class="px-4 py-2 border border-gray-300">
              {{ feature.properties?.[key] ?? '' }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts">
import 'leaflet/dist/leaflet.css'
import { LGeoJson, LMap, LTileLayer } from '@vue-leaflet/vue-leaflet'
import shp from 'shpjs'
import type { Feature, GeoJsonProperties, Geometry } from 'geojson'
import {
  type GeoJSONOptions,
  type LatLngBoundsExpression,
  type Layer,
  type PointTuple,
} from 'leaflet'
import L from 'leaflet'

export default {
  components: {
    LGeoJson,
    LMap,
    LTileLayer,
  },

  data() {
    return {
      zoom: 2,
      center: [47.41322, -1.219482] as PointTuple,
      schema: [] as string[],
      file: null as File | null,
      geojsonFeatures: null as Feature<Geometry, GeoJsonProperties>[] | null,
      bbox: null as LatLngBoundsExpression | null,
      map: null as L.Map | null,
    }
  },

  watch: {
    async bbox(newVal: LatLngBoundsExpression | null) {
      if (newVal && this.map) {
        this.map.fitBounds(newVal)
      }
    },
  },

  methods: {
    onMapReady(mapInstance: L.Map) {
      this.map = mapInstance
    },

    async handleUpload(event: Event) {
      const input: HTMLInputElement = event.target as HTMLInputElement
      if (!input.files || input.files.length === 0) {
        return
      }

      const file: File = input.files[0]
      if (file.type !== 'application/zip') {
        alert('Please upload a valid zipped shp')
        return
      }

      this.file = file

      try {
        const arrayBuffer: ArrayBuffer = await file.arrayBuffer()
        const geojsonData: shp.FeatureCollectionWithFilename | shp.FeatureCollectionWithFilename[] =
          await shp(arrayBuffer)

        let features: Feature<Geometry, GeoJsonProperties>[] = []

        if (Array.isArray(geojsonData)) {
          if (geojsonData.length > 0) {
            features = geojsonData[0].features

            const layer = L.geoJSON(geojsonData[0])
            this.bbox = [
              [layer.getBounds().getSouth(), layer.getBounds().getWest()],
              [layer.getBounds().getNorth(), layer.getBounds().getEast()],
            ]
          }
        } else {
          features = geojsonData.features

          const layer = L.geoJSON(geojsonData)
          this.bbox = [
            [layer.getBounds().getSouth(), layer.getBounds().getWest()],
            [layer.getBounds().getNorth(), layer.getBounds().getEast()],
          ]
        }

        if (features.length > 0) {
          this.schema = Object.keys(features[0].properties || {})
          this.geojsonFeatures = features
        } else {
          this.schema = []
          alert('Failed to detect schema')
        }
      } catch (error) {
        console.error(error)
        alert('Failed to read file')
      }
    },

    async handleSave() {
      try {
        const response = await fetch('/api/file/save', {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/zip',
          },
          body: this.file,
        });

        if (!response.ok) {
          throw new Error(`server error: ${response.status}`);
        }
      } catch (error) {
        console.error(error);
        alert('Failed to save file')
      }
    },
  },

  computed: {
    geoJsonOptions(): GeoJSONOptions {
      return {
        onEachFeature: (feature: Feature<Geometry, GeoJsonProperties>, layer: Layer) => {
          layer.on('click', () => {
            const properties = feature.properties
            const content = Object.entries(properties || {})
              .map(([key, value]) => `<strong>${key}:</strong> ${value}`)
              .join('<br>')

            layer
              .bindTooltip(content, {
                sticky: true,
                direction: 'top',
                opacity: 0.9,
              })
              .openTooltip()
          })
        },
      }
    },
  },
}
</script>

<style></style>
