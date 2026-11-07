cpu 8086
bits 16          ; Diz ao NASM para gerar instruções de 16 bits

start:
    mov ah, 0x0E ; Função de teletexto da BIOS (imprimir caractere)
    mov al, 'H'  ; Caractere a ser impresso
    int 0x10     ; Interrupção de vídeo da BIOS

    mov al, 'i'
    int 0x10

    cli          ; Desativa interrupções
.loop:
    hlt          ; Para o processador
    jmp .loop    ; Loop infinito de segurança
