ENTRY(_start)

PHDRS
{
    text PT_LOAD FLAGS(5);
    data PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = 0xffffffff80000000;

    .text : {
        *(.text*)
    } :text

    . = ALIGN(4096);

    .data : {
        *(.data*)
    } :data
}
