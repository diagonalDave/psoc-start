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
    __enable_irq(); /* Enable global interrupts. */

    start_cm4();
}

/* [] END OF FILE */
