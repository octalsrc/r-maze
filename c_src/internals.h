/*
 *  Original notice:
 *
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

#ifndef SEEN_INTERNALS_H
#define SEEN_INTERNALS_H

#include "directionals.h"

enum tiletype { WALL, SPACE };

struct mazetile
{
    enum tiletype t;
    float light;
    struct DR_position p;
};
struct maze
{
    struct mazetile *tiles;
    struct DR_position start_position;
    struct DR_position goal_position;
    int size;
};

enum tiletype get_tiletype ( struct mazetile *tiles, int size, struct DR_position pos );
struct mazetile new_mazetile ( enum tiletype type, int x, int y );
struct maze* new_maze_pointer ( int size, struct mazetile *tiles, struct DR_position start_position, struct DR_position goal_position );
void destroy_maze ( struct maze *maze );

#endif
