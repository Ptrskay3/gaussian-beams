## Python package for calculating Gaussian Beams.

#### Work in progress!

Here is a minimal example:

```python
from beams import GaussianBeam
import matplotlib.pyplot as plt

beam = GaussianBeam.new_with_offset(632.8 * 10E-9, 1E-2, 0.8, 3500.0, 3500.0)
# render on a 4000 x 4000 pixel grid
grid = beam.meshgrid(4000, 4000)
plt.imshow(grid, cmap="gray")
```
