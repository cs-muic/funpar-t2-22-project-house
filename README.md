# funpar-t2-22-project-house.

# This Project is about regenerating the image from the input image using rectangle in parallel.

## The Planning Stage

Our goal is to create an image by creating rectangles at random positions with random colors and random sizes.
We planned to use edge detection and concurrency to speed up the generation of our output.

## How the Code Works

1.) We take in the path to the target image through the terminal
2.) We create a canvas with the same size as the target image
3.) We extract the colors in the target image
4.) We display the canvas on to a window
5.) We calculate the original cost between our canvas and the target image
6.) We run an event loop which does the following:
      Create n number of images concurrently
      Find the cost reduction of each image
      Get the image with the highest cost reduction
      If the cost reduction is positive, draw that shape onto the canvas and reduce the current cost by the cost reduction
      
## The Problems During the Process

After we came up with our first iteration of the edge detection function,
we tried to test our code to run by randomly selecting the coordinates on the edges versus randomly generated coordinates.
So to measure this difference, we used the cost function.
When we used edge detection and set the possible shapes concurrently generated at 3000 images, at 10 seconds the average cost was 534231.
However, when we did not use edge detection and set the shapes concurrently generated at 3000 images, at 10 seconds the average cost was 151151.
Therefore, we concluded that edge detection wasn’t helping.





