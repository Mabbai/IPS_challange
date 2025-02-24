# IPS_challange

Brief algorithm description:
The algorithm sorts each point into a cube-shaped cell defined by three integers, with side length equal to the distance we search pairs within. It then processes each cell by counting the number of eligible pairs of points internally and the number of eligible pairs between the points inside the cell and points inside a selected constellation of surrounding cells. The constellation prevents double counting and is selected based on a cell having odd or even coordinates.
