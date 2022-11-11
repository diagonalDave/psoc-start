/* ========================================
 * Author Dave Humphries 2022
 * Use under one of the following licences:
    1. Anything goes, use as you wish.
    2. MIT licence
    3. Apache 2.0 Licence
 * ========================================
*/
#include "project.h"
#include "start.h"

int main(void)
{
   /*
        start_cm4 is defined in the start rust crate. The start project
        directory should be at the root of the psoc-start directory. 
        The psoc-creator project has been configured to look in the normal rust target
        directories for the crate binary. Check src/lib.rs for the code.
        And the README.md for information on building this project.
    */
    start_cm0();
}

/* [] END OF FILE */
