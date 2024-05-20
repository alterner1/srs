package main

import (
 "fmt"
 "image"
 "image/color"
 "image/png"
 "math/rand"
 "os"
 "time"
)

// Генотип содержит 16 генов
type Genotype struct {
 Genes [16]int
}

// Создаем случайный генотип
func RandomGenotype() Genotype {
 rand.Seed(time.Now().UnixNano())
 var genes [16]int
 for i := 0; i < 15; i++ {
  genes[i] = rand.Intn(19) - 9 // Значения от -9 до +9
 }
 genes[15] = rand.Intn(11) + 2 // Значения от 2 до 12
 return Genotype{Genes: genes}
}

// Функция для мутации генотипа
func Mutate(genotype Genotype) Genotype {
 rand.Seed(time.Now().UnixNano())
 index := rand.Intn(16)
 if index < 15 {
  genotype.Genes[index] += rand.Intn(3) - 1 // Изменение на -1, 0 или +1
  if genotype.Genes[index] > 9 {
   genotype.Genes[index] = 9
  } else if genotype.Genes[index] < -9 {
   genotype.Genes[index] = -9
  }
 } else {
  genotype.Genes[15] += rand.Intn(3) - 1 // Изменение на -1, 0 или +1
  if genotype.Genes[15] > 12 {
   genotype.Genes[15] = 12
  } else if genotype.Genes[15] < 2 {
   genotype.Genes[15] = 2
  }
 }
 return genotype
}

// Функция для генерации изображения биоморфы
func GenerateBiomorph(genotype Genotype) *image.Gray {
 img := image.NewGray(image.Rect(0, 0, 150, 150))
 centerX, centerY := 75, 75

 length := genotype.Genes[15]
 for i := 0; i < 7; i++ {
  x := centerX
  y := centerY
  for j := 0; j < length; j++ {
   img.SetGray(x, y, color.Gray{255})
   x += genotype.Genes[i*2]
   y += genotype.Genes[i*2+1]
   img.SetGray(150-x, y, color.Gray{255}) // Симметрия
   img.SetGray(x, 150-y, color.Gray{255}) // Симметрия
   img.SetGray(150-x, 150-y, color.Gray{255}) // Симметрия
  }
 }
 return img
}

// Сохранение изображения в файл
func SaveImage(img *image.Gray, filename string) error {
 file, err := os.Create(filename)
 if err != nil {
  return err
 }
 defer file.Close()
 return png.Encode(file, img)
}

func main() {
 genotype := RandomGenotype()
 fmt.Println("Initial Genotype:", genotype)
 for generation := 0; generation < 10; generation++ {
  fmt.Println("Generation:", generation)
  for i := 0; i < 10; i++ { // Порождаем N созданий (10 в данном случае)
   mutatedGenotype := Mutate(genotype)
   img := GenerateBiomorph(mutatedGenotype)
   filename := fmt.Sprintf("biomorph_gen%d_ind%d.png", generation, i)
   err := SaveImage(img, filename)
   if err != nil {
    fmt.Println("Error saving image:", err)
   }
   fmt.Println("Saved:", filename)
  }
 }
}