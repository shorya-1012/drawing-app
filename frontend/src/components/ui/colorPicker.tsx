'use client'

import { Button } from './button'
import { Input } from './input'
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from './popover'
import { Tabs, TabsContent, TabsList, TabsTrigger } from './tabs'
import { cn } from '../../lib/utils'
import { Paintbrush } from 'lucide-react'
import { Link } from 'react-router-dom'
import { useMemo } from 'react'

export function GradientPicker({
    background,
    setBackground,
    className,
}: {
    background: string
    setBackground: (background: string) => void
    className?: string
}) {
    const solids = [
        '#E2E2E2',
        '#ff75c3',
        '#ffa647',
        '#ffe83f',
        '#9fff5b',
        '#70e2ff',
        '#cd93ff',
        '#09203f',
        '#FF5733', // Fiery Red
        '#33FF57', // Lime Green
        '#3357FF', // Vivid Blue
        '#FF33A1', // Bright Pink
        '#33FFF1', // Aqua Blue
        '#F1FF33', // Lemon Yellow
        '#FFB733'
    ]

    const defaultTab = useMemo(() => {
        if (background.includes('url')) return 'image'
        if (background.includes('gradient')) return 'gradient'
        return 'solid'
    }, [background])

    return (
        <Popover>
            <PopoverTrigger asChild>
                <Button
                    variant={'outline'}
                    className={cn(
                        'w-[220px] justify-start text-left font-normal',
                        !background && 'text-muted-foreground',
                        className
                    )}
                >
                    <div className="w-full flex items-center gap-2">
                        {background ? (
                            <div
                                className="h-4 w-4 rounded !bg-center !bg-cover transition-all"
                                style={{ background }}
                            ></div>
                        ) : (
                            <Paintbrush className="h-4 w-4" />
                        )}
                        <div className="truncate flex-1">
                            {background ? background : 'Pick a color'}
                        </div>
                    </div>
                </Button>
            </PopoverTrigger>
            <PopoverContent className="w-64">
                <Tabs defaultValue={defaultTab} className="w-full">
                    <TabsList className="w-full mb-4">
                        <TabsTrigger className="flex-1 w-full" value="solid">
                            Solid
                        </TabsTrigger>
                    </TabsList>

                    <TabsContent value="solid" className="flex flex-wrap gap-1 mt-0">
                        {solids.map((s) => (
                            <div
                                key={s}
                                style={{ background: s }}
                                className="rounded-md h-6 w-6 cursor-pointer active:scale-105"
                                onClick={() => setBackground(s)}
                            />
                        ))}
                    </TabsContent>
                </Tabs>

                <Input
                    id="custom"
                    value={background}
                    className="col-span-2 h-8 mt-4"
                    onChange={(e) => setBackground(e.currentTarget.value)}
                />
            </PopoverContent>
        </Popover>
    )
}
