package main

import (
 "crypto/rand"
 "fmt"
 "image"
 "image/color"
 "image/draw"
 "image/png"
 "math"
 "math/big"
 "os"
)

const (
 imageSize       = 150
 nBiomorphs      = 16
 genotypeLength  = 16
 maxGenerations  = 1000
 stagnationLimit = 10
)

type Genotype [genotypeLength]int

func randomGenotype() Genotype {
 var genotype Genotype
 for i := 0; i < genotypeLength-1; i++ {
  genotype[i] = randInt(-9, 9)
 }
 genotype[genotypeLength-1] = randInt(2, 12)
 return genotype
}

func mutate(genotype Genotype) Genotype {
 mutated := genotype
 geneIndex := randInt(0, genotypeLength-1)
 if geneIndex == genotypeLength-1 {
  mutated[geneIndex] += randInt(-1, 1)
  if mutated[geneIndex] < 2 {
   mutated[geneIndex] = 2
  } else if mutated[geneIndex] > 12 {
   mutated[geneIndex] = 12
  }
 } else {
  mutated[geneIndex] += randInt(-1, 1)
  if mutated[geneIndex] < -9 {
   mutated[geneIndex] = -9
  } else if mutated[geneIndex] > 9 {
   mutated[geneIndex] = 9
  }
 }
 return mutated
}

func generateImage(genotype Genotype) image.Image {
 img := image.NewRGBA(image.Rect(0, 0, imageSize, imageSize))
 draw.Draw(img, img.Bounds(), &image.Uniform{color.White}, image.Point{}, draw.Src)

 length := genotype[genotypeLength-1]
 centerX, centerY := imageSize/2, imageSize/2

 for i := 0; i < genotypeLength-1; i += 2 {
  x1 := centerX + length*genotype[i]
  y1 := centerY + length*genotype[i+1]
  x2 := centerX - length*genotype[i]
  y2 := centerY - length*genotype[i+1]

  drawLine(img, centerX, centerY, x1, y1, color.Black)
  drawLine(img, centerX, centerY, x2, y2, color.Black)
  centerX, centerY = x1, y1
 }

 return img
}

func drawLine(img *image.RGBA, x0, y0, x1, y1 int, col color.Color) {
 dx := abs(x1 - x0)
 dy := abs(y1 - y0)
 sx := -1
 if x0 < x1 {
  sx = 1
 }
 sy := -1
 if y0 < y1 {
  sy = 1
 }
 err := dx - dy

 for {
  img.Set(x0, y0, col)
  if x0 == x1 && y0 == y1 {
   break
  }
  e2 := err * 2
  if e2 > -dy {
   err -= dy
   x0 += sx
  }
  if e2 < dx {
   err += dx
   y0 += sy
  }
 }
}

func abs(x int) int {
 if x < 0 {
  return -x
 }
 return x
}

func similarity(img1, img2 image.Image) float64 {
 bounds := img1.Bounds()
 var sum float64

 for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
  for x := bounds.Min.X; x < bounds.Max.X; x++ {
   r1, g1, b1, _ := img1.At(x, y).RGBA()
   r2, g2, b2, _ := img2.At(x, y).RGBA()
   sum += math.Abs(float64(r1-r2)) + math.Abs(float64(g1-g2)) + math.Abs(float64(b1-b2))
  }
 }

 return sum / float64(bounds.Dx()*bounds.Dy())
}

func saveImage(img image.Image, filename string) error {
 file, err := os.Create(filename)
 if err != nil {
  return err
 }
 defer file.Close()

 return png.Encode(file, img)
}

func randInt(min int, max int) int {
 nBig, _ := rand.Int(rand.Reader, big.NewInt(int64(max-min+1)))
 return min + int(nBig.Int64())
}

func main() {
 targetFileName := "target.png"
 targetFile, err := os.Open(targetFileName)
 if err != nil {
  fmt.Println("Error opening target file:", err)
  return
 }
 defer targetFile.Close()

 targetImg, err := png.Decode(targetFile)
 if err != nil {
  fmt.Println("Error decoding target image:", err)
  return
 }

 genotypes := make([]Genotype, nBiomorphs)
 for i := range genotypes {
  genotypes[i] = randomGenotype()
 }

 bestGenotype := genotypes[0]
 bestSimilarity := similarity(generateImage(bestGenotype), targetImg)
 stagnationCounter := 0

 for generation := 0; generation < maxGenerations && stagnationCounter < stagnationLimit; generation++ {
  newGenotypes := make([]Genotype, nBiomorphs)

  for i := range newGenotypes {
   newGenotypes[i] = mutate(bestGenotype)
   img := generateImage(newGenotypes[i])
   filename := fmt.Sprintf("./images/generation_%03d_biomorph_%02d.png", generation, i)
   saveImage(img, filename)

   similarityScore := similarity(img, targetImg)
   if similarityScore < bestSimilarity {
    bestSimilarity = similarityScore
    bestGenotype = newGenotypes[i]
    stagnationCounter = 0
   } else {
    stagnationCounter++
   }
  }

  fmt.Printf("Generation %d: Best similarity %f\\n", generation, bestSimilarity)

        if stagnationCounter >= stagnationLimit {
            fmt.Println("Stagnation limit reached. Ending evolution.")
            break
        }
    }

 finalImg := generateImage(bestGenotype)
 finalFilename := "final_biomorph.png"
 saveImage(finalImg, finalFilename)
 fmt.Println("Final biomorph saved as", finalFilename)
}
