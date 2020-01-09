/*
 *  << c-maze, a simple generated maze crawler written in C >>
 *  Copyright (C) 2013 Nick Lewchenko
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 * 
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

#ifndef SEEN_DIRECTIONALS_H
#define SEEN_DIRECTIONALS_H

enum DR_direction { NORTH, NORTHEAST, EAST, SOUTHEAST, SOUTH, SOUTHWEST, WEST, NORTHWEST, FULL_CYCLE };
enum DR_orientation { FRONT, FRONTRIGHT, RIGHT, BACKRIGHT, BACK, BACKLEFT, LEFT, FRONTLEFT };

struct DR_position
{
    int x;
    int y;
};


struct DR_position DR_new_position ( int x, int y );
int DR_equal_pos ( struct DR_position a, struct DR_position b );

struct DR_position DR_get_adj ( struct DR_position this_pos, enum DR_direction dir );
enum DR_direction DR_get_rel ( enum DR_direction this_dir, enum DR_orientation offset );

#endif
