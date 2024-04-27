import scala.collection.mutable.ArrayBuffer

class Model(var value: Int) {

  def duplicate(): Unit = this.value = this.value * 2;

}

object Main {

  private def doBenchmark(quantity: Int): Unit = {
    var startTime = System.nanoTime()
    val buffer = new ArrayBuffer[Model](quantity)
    0.until(quantity).foreach { i =>
      buffer.append(new Model(i))
    }
    println("Time to fill buffer: %d nanos".format(System.nanoTime() - startTime))
    val iteratedList = buffer.toList

    startTime = System.nanoTime()
    iteratedList.foreach { i =>
      i.duplicate()
    }
    println("Time to duplicate contents of all: %d nanos".format(System.nanoTime() - startTime))
  }

  def main(args: Array[String]): Unit = {


    println("Please input quantity of objects to create into a list instead of array:")
    val quantity = scala.io.StdIn.readInt()
    assert(quantity > 0)


    println("Cold Start:")
    doBenchmark(quantity)

    println("\nWarmed up, press ENTER key to continue")
    scala.io.StdIn.readLine()
    doBenchmark(quantity)


  }
}
