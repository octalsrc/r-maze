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

#include "maze_gen.h"
#include "internals.h"
#include "directionals.h"

struct probs
{
    int twisty; /* Probability of changing direction */
    int swirly; /* Probability of going right when changing direction (so non-swirly would be 0.5) */
    int branchy; /* Probability of branching */
};

struct path_head
{
    struct DR_position p;
    enum DR_direction d;
    int g; /* growth probability */
    int still_space;

    struct path_head *next;
};

int count_paths ( struct path_head *last )
{
    int c;

    if ( last->next == NULL )
    {
        c = 0;
    }
    else
    {
        c = count_paths ( last->next );
    }

    return c + 1;
}

struct path_head *get_path ( struct path_head *last, int i )
{
    struct path_head *result;

    if ( i < 0 )
    {
        result = last;
    }
    else
    {
        result = get_path ( last->next, i-1 );
    }

    return result;
}

struct path_head *new_path ( struct DR_position p, enum DR_direction d, int g )
{
    struct path_head *path;
    path = malloc ( sizeof ( struct path_head ) );
    path->p = p;
    path->d = d;
    path->g = g;
    path->still_space = 1;
    path->next = NULL;

    return path;
}

void add_path ( struct path_head *last, struct path_head *addition )
{
    if ( last->next == NULL )
    {
        last->next = addition;
    }
    else
    {
        add_path ( last->next, addition );
    }
}

void destroy_paths ( struct path_head *last )
{
    if ( last->next != NULL )
    {
        destroy_paths ( last->next );
    }

    free ( last );
}

int check_space ( struct mazetile *tiles, int size, struct DR_position pos, enum DR_direction d )
{
    int result = 1;
    int x;
    enum DR_direction check_dir;

    if ( pos.x == 0 || pos.x == size-1 || pos.y == 0 || pos.y == size-1 )
    {
        result = 0;
    }
    else
    {
        for ( x = 0; x < 5; x++ )
        {
            check_dir = DR_get_rel ( DR_get_rel ( d, LEFT ), x );
            if ( get_tiletype ( tiles, size, DR_get_adj ( pos, check_dir ) ) != WALL )
            {
                result = 0;
            }
        }
    }

    return result;
}
int check_all ( struct mazetile *tiles, int size, struct DR_position pos, enum DR_direction d )
{
    int result = 0;
    int f, l, r;

    f = check_space ( tiles, size, DR_get_adj ( pos, d ), d );
    l = check_space ( tiles, size, DR_get_adj ( pos, DR_get_rel ( d, LEFT ) ), DR_get_rel ( d, LEFT ) );
    r = check_space ( tiles, size, DR_get_adj ( pos, DR_get_rel ( d, RIGHT ) ), DR_get_rel ( d, RIGHT ) );

    if ( f || l || r )
    {
        result = 1;
    }

    return result;
}
int try_move ( struct mazetile *tiles, int size, struct path_head *last, struct DR_position try_pos, enum DR_direction dir )
{
    int result = 0;

    if ( check_space ( tiles, size, try_pos, dir ) )
    {
        tiles[try_pos.x * size + try_pos.y].t = SPACE;
        last->p = try_pos;
        last->d = dir;
        result = 1;
    }

    return result;
}

void try_branch ( struct mazetile *tiles, int size, struct path_head *last, enum DR_orientation o, struct probs probs )
{
    int r;
    int i;
    struct path_head *new_branch;

    for ( i = 0; i < 2; i++ )
    {
    new_branch = NULL;
    r = ( ( int ) rand () ) % 100;
    if ( r < probs.branchy )
    {
        r = ( ( int ) rand () ) % 100;
        switch ( i )
        {
            case 0:
        switch ( o )
        {
            case FRONT:
                new_branch = new_path ( last->p, DR_get_rel ( last->d, RIGHT ), r );
                break;
            case LEFT:
                new_branch = new_path ( last->p, DR_get_rel ( last->d, FRONT ), r );
                break;
            case RIGHT:
                new_branch = new_path ( last->p, DR_get_rel ( last->d, LEFT ), r );
                break;
            default:
                break;
        }
                break;
            case 1:
        switch ( o )
        {
            case FRONT:
                new_branch = new_path ( last->p, DR_get_rel ( last->d, LEFT ), r );
                break;
            case LEFT:
                new_branch = new_path ( last->p, DR_get_rel ( last->d, RIGHT ), r );
                break;
            case RIGHT:
                new_branch = new_path ( last->p, DR_get_rel ( last->d, FRONT ), r );
                break;
            default:
                break;
        }
                break;
            default:
                break;
        }
        if ( try_move ( tiles, size, new_branch, DR_get_adj ( new_branch->p, new_branch->d ), new_branch->d ) )
        {
            add_path ( last, new_branch );
            /* printf ( "added path at %d, %d with direction %d\n", new_branch->p.x, new_branch->p.y, new_branch->d ); */
        }
    }
    }
}



int advance_path ( struct mazetile *tiles, int size, struct path_head *last, struct probs probs )
{
    enum DR_orientation o;
    enum DR_direction dir;
    struct DR_position try_pos;
    int r;
    int result = 0;

    if ( check_all ( tiles, size, last->p, last->d ) )
    {
        result = 1;

        r = ( ( int ) rand () ) % 100;
        if ( r < last->g )
        {
            r = ( ( int ) rand () ) % 100;
            if ( r < probs.twisty )
            {
                o = FRONT;
            }
            else
            {
                r = ( ( int ) rand () ) % 100;
                if ( r < probs.swirly )
                {
                    o = RIGHT;
                }
                else
                {
                    o = LEFT;
                }
            }
            dir = DR_get_rel ( last->d, o );
            try_pos = DR_get_adj ( last->p, dir );
    
            try_branch ( tiles, size, last, o, probs );
            try_move ( tiles, size, last, try_pos, dir );

        }
    }
    else
    {
        last->still_space = 0;
    }

    return result;
}
        
        

int iterate_paths (struct mazetile *tiles, int size, struct path_head *last, struct probs probs, int *count )
{
    int i = 0;
    int j = 0;

    if ( *count > 1000000 )
    {
        *count = 0;
        printf ( "." );
        fflush ( stdout );
    }
    else
    {
        *count += 1;
    }

    if ( last->still_space )
    {
        i = advance_path ( tiles, size, last, probs );
    }
    if ( last->next != NULL )
    {
        j = iterate_paths ( tiles, size, last->next, probs, count );
    }

    return ( i || j );
}



struct maze *generate_maze ( int size, int twisty, int swirly, int branchy )
{
    int x;
    int y;
    int r;
    int count = 0;
    int space = 1;
    struct mazetile *tiles;
    struct probs probs;
    struct DR_position start_position;
    struct DR_position goal_position;
    struct path_head *root;

    /*  Original probabilites were
    probs.twisty = 70;
    probs.swirly = 50;
    probs.branchy = 30;
    */

    probs.twisty = twisty;
    probs.swirly = swirly;
    probs.branchy = branchy;


    if ( size < 10 )
    {
        printf ( "maze size must be at least 10,\n or else you wouldn't have any space!\n" );
    }
    tiles = malloc ( sizeof ( struct mazetile ) * size * size );

    /*
     * Initialize the entire maze as walls, from which we will carve paths
     */
    for ( x = 0; x < size; x++ )
    {
        for ( y = 0; y < size; y++ )
        {
            tiles[x * size + y] = new_mazetile ( WALL, x, y );
        }
    }

    x = ( int ) ( rand () / ( double ) RAND_MAX * ( ( double ) size - 6 ) ) + 3;
    y = ( int ) ( rand () / ( double ) RAND_MAX * ( ( double ) size - 6 ) ) + 3;
    start_position = DR_new_position ( x, y );
    printf ( "Starting position at %d, %d\n", x, y );
    tiles[x * size + y].t = SPACE;

    root = new_path ( start_position, SOUTH, 80 );
    printf ( "root created\n" );

    printf ( "Generating maze ... " );
    x=0;
    while ( space && x < 5000 )
    {
        space = iterate_paths ( tiles, size, root, probs, &count );
        x++;
    }
    printf ( "\n" );

    x = ( int ) ( rand () / ( double ) RAND_MAX * ( ( double ) count_paths ( root ) ) );

    goal_position = get_path ( root, x-1 )->p;

    printf ( "finished, destroying paths\n" );

    destroy_paths ( root );

    return new_maze_pointer ( size, tiles, start_position, goal_position );
}
