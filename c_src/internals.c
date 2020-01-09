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

#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

#include "internals.h"
#include "directionals.h"

enum recurse_light_class { PRIMARY, SEC_RIGHT, SEC_LEFT };

struct player new_player ( struct DR_position start_pos );
struct mazetile new_mazetile ( enum tiletype type, int x, int y );
void destroy_maze ( struct maze *maze );

struct mazegame new_mazegame ( struct maze *maze )
{
    struct mazegame game;
    
    game.player = new_player ( maze->start_position );
    game.maze = maze;
    
    return game;
}

void destroy_mazegame ( struct mazegame *game )
{
    destroy_maze ( game->maze );
    free ( game->maze );
}
void destroy_maze ( struct maze *maze )
{
    free ( maze->tiles );
}

struct player new_player ( struct DR_position start_pos )
{
    struct player player;
    player.p = start_pos;
    player.d = SOUTH;
    player.battery = 8000;
    player.flashlight_switch = 1;
    
    return player;
}
struct maze* new_maze_pointer ( int size, struct mazetile *tiles, struct DR_position start_position, struct DR_position goal_position )
{
    struct maze *maze;
    maze = malloc ( sizeof ( struct maze ) );
    maze->size = size;
    maze->tiles = tiles;
    maze->start_position = start_position;
    maze->goal_position = goal_position;

    return maze;
}

struct mazetile new_mazetile ( enum tiletype type, int x, int y )
{
    struct mazetile mazetile;
    mazetile.t = type;
    mazetile.p = DR_new_position ( x, y );
    mazetile.light = 0;

    return mazetile;
}

enum tiletype get_tiletype ( struct mazetile *tiles, int size, struct DR_position pos )
{
    return tiles[pos.x * size + pos.y].t;
}

void reset_light ( struct mazegame *game )
{
    int x;
    int y;
    int s = game->maze->size;
    for ( x = 0; x < s; x++ )
    {
        for ( y = 0; y < s; y++ )
        {
            game->maze->tiles[ x * s + y ].light = 0;
        }
    }
}

void recurse_light ( float value, struct maze *maze, enum DR_direction d, enum recurse_light_class o, struct DR_position p )
{
    /*printf ( "starting recurse light with %f\n", value );*/
    float divisor = 2.0;
    float side_divisor = 3.0;
    int s = maze->size;
    maze->tiles[p.x * s + p.y].light += value;
    if ( value >= 1.0 && get_tiletype ( maze->tiles, s, p ) == SPACE )
    {
        switch ( o )
        {
            case PRIMARY:
                recurse_light ( value / divisor, maze, d, PRIMARY, DR_get_adj ( p, d ) );
                recurse_light ( value / side_divisor, maze, d, SEC_LEFT, DR_get_adj ( p, DR_get_rel ( d, FRONTLEFT ) ) );
                recurse_light ( value / side_divisor, maze, d, SEC_RIGHT, DR_get_adj ( p, DR_get_rel ( d, FRONTRIGHT ) ) );
                break;
            case SEC_LEFT:
                recurse_light ( value / side_divisor, maze, d, SEC_LEFT, DR_get_adj ( p, DR_get_rel ( d, FRONTLEFT ) ) );
                break;
            case SEC_RIGHT:
                recurse_light ( value / side_divisor, maze, d, SEC_RIGHT, DR_get_adj ( p, DR_get_rel ( d, FRONTRIGHT ) ) );
                break;
            default:
                break;
        }
    }
}

enum command assoc( enum DR_direction d ) {
  enum command c;
  
  switch( d ) {
  case NORTH:
    c = MOVE_N;
    break;
  case SOUTH:
    c = MOVE_S;
    break;
  case WEST:
    c = MOVE_W;
    break;
  case EAST:
    c = MOVE_E;
    break;
  default:
    c = NONE;
    break;
  }

  return c;
}

int move_player( struct mazegame *g, const enum DR_direction d ) {
  int s = g->maze->size;
  int moved = 0;

  /* the adjacent position */
  struct DR_position p = DR_get_adj( g->player.p, d );

  /* the tiletype of the adjacent position */
  enum tiletype t = get_tiletype( g->maze->tiles, s, p );
  
  if( d != g->player.d ) {
    g->player.d = d;
    moved = 1;
  } else if( SPACE == t ) {
    g->player.p = p;
    moved = 1;
  }

  return moved;
}

int check_around( const struct mazegame *g, const enum DR_orientation o ) {
  struct player p = g->player;
  enum tiletype t =
    get_tiletype( g->maze->tiles, g->maze->size, DR_get_adj( p.p, DR_get_rel( p.d, o ) ) );

  return( SPACE == t );
}

int turn_player( struct mazegame *g ) {
  int turned = 0;

  int left = check_around( g, LEFT );
  int right = check_around( g, RIGHT );

  if( left && !right ) {
    g->player.d = DR_get_rel( g->player.d, LEFT );
    turned = 1;
  } else if( right && !left ) {
    g->player.d = DR_get_rel( g->player.d, RIGHT );
    turned = 1;
  }

  return turned;
}

void set_switch( struct mazegame *g, int v ) {
  g->player.flashlight_switch = v;
}

void flip_switch( struct mazegame *g ) {
  if( 1 == g->player.flashlight_switch ) {
    set_switch( g, 8 );
  } else {
    set_switch( g, 1 );
  }
}

void update_game( struct mazegame *g, enum command player_move ) {
  int s = g->maze->size;

  int power = g->player.battery * g->player.flashlight_switch * 4;

  switch( player_move ) {

    case FLASHLIGHT:
      flip_switch( g );
      break;

    case RUN:
      if( !move_player( g, g->player.d ) ) {
        turn_player( g );
      }
      break;
    case MOVE_N:
      move_player( g, NORTH );
      break;
    case MOVE_S:
      move_player( g, SOUTH );
      break;
    case MOVE_W:
      move_player( g, WEST );
      break;
    case MOVE_E:
      move_player( g, EAST );
      break;
    default:
      break;
  }

  reset_light( g );
  recurse_light( power / 1600, g->maze, g->player.d, PRIMARY, g->player.p );

  g->player.battery -= g->player.flashlight_switch;
}
