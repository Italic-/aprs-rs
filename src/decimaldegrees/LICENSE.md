% PyDecimalDegrees - geographic coordinates conversion utility.

Copyright (C) 2006-2013 by Mateusz Łoskot <mateusz@loskot.net>
Copyright (C) 2010-2013 by Evan Wheeler <ewheeler@unicef.org>

This file is part of PyDecimalDegrees module.

This software is provided 'as-is', without any express or implied warranty.
In no event will the authors be held liable for any damages arising from
the use of this software.

Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it
freely,
subject to the following restrictions:
1. The origin of this software must not be misrepresented; you must not
   claim that you wrote the original software. If you use this software
   in a product, an acknowledgment in the product documentation would be
   appreciated but is not required.
2. Altered source versions must be plainly marked as such, and must not be
   misrepresented as being the original software.
3. This notice may not be removed or altered from any source distribution.

DESCRIPTION

DecimalDegrees module provides functions to convert between
degrees/minutes/seconds and decimal degrees.

Original source
distribution:
http://mateusz.loskot.net/software/gis/pydecimaldegrees/

Inspired by Walter Mankowski's Geo::Coordinates::DecimalDegrees module
for Perl, originally located in CPAN Archives:
http://search.cpan.org/~waltman/Geo-Coordinates-DecimalDegrees-0.05/

Examples are based on the following coordinates:
DMS: 37 46' 26"
   -122 25' 52"
DM: 37 46.433258'
   -122 25.866394'
DD: 37.773888
   -122.43111

Port to Rust and resulting modifications by Jeffrey "italic" Hoover
