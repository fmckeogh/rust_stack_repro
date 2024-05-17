#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use UsingAArch32::*;
use SPSR_svc_read::*;
use SPSR_hyp_read::*;
use Unreachable::*;
use common::*;
pub fn SPSR_read<T: Tracer>(state: &mut State, tracer: &T, N: i128) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        ga_9117: ProductType700c18a878c5601b,
        ga_9121: u8,
        ga_9110: u8,
        result: Bits,
        ga_9113: ProductType700c18a878c5601b,
        N: i128,
    }
    let fn_state = FunctionState {
        N,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b10 b1
        if s_0_1 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #64s : i
        let s_1_0: i128 = 64;
        // D s_1_1: read-var N:i
        let s_1_1: i128 = fn_state.N;
        // D s_1_2: cmp-eq s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) == (s_1_0));
        // N s_1_3: assert s_1_2
        let s_1_3: () = assert!(s_1_2);
        // C s_1_4: const #16975u : u32
        let s_1_4: u32 = 16975;
        // D s_1_5: read-reg s_1_4:u8
        let s_1_5: u8 = {
            let value = state.read_register::<u8>(s_1_4 as isize);
            tracer.read_register(s_1_4 as isize, value);
            value
        };
        // D s_1_6: write-var ga#9121 <= s_1_5
        fn_state.ga_9121 = s_1_5;
        // D s_1_7: read-var ga#9121:u8
        let s_1_7: u8 = fn_state.ga_9121;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 2u16);
        // C s_1_9: const #440u : u32
        let s_1_9: u32 = 440;
        // D s_1_10: read-reg s_1_9:u8
        let s_1_10: u8 = {
            let value = state.read_register::<u8>(s_1_9 as isize);
            tracer.read_register(s_1_9 as isize, value);
            value
        };
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 2u16);
        // D s_1_12: cmp-eq s_1_8 s_1_11
        let s_1_12: bool = ((s_1_8) == (s_1_11));
        // D s_1_13: not s_1_12
        let s_1_13: bool = !s_1_12;
        // N s_1_14: branch s_1_13 b5 b2
        if s_1_13 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_2_0: const #90648u : u32
        let s_2_0: u32 = 90648;
        // D s_2_1: read-reg s_2_0:u64
        let s_2_1: u64 = {
            let value = state.read_register::<u64>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // C s_2_3: const #64s : i
        let s_2_3: i128 = 64;
        // D s_2_4: cast zx s_2_1 -> bv
        let s_2_4: Bits = Bits::new(s_2_1 as u128, 64u16);
        // D s_2_5: bit-extract s_2_4 s_2_2 s_2_3
        let s_2_5: Bits = (Bits::new(
            ((s_2_4) >> (s_2_2)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_6: write-var result <= s_2_5
        fn_state.result = s_2_5;
        // N s_2_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var result:bv
        let s_4_0: Bits = fn_state.result;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_5_0: read-var ga#9121:u8
        let s_5_0: u8 = fn_state.ga_9121;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #432u : u32
        let s_5_2: u32 = 432;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // D s_5_6: not s_5_5
        let s_5_6: bool = !s_5_5;
        // N s_5_7: branch s_5_6 b7 b6
        if s_5_6 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_6_0: const #15736u : u32
        let s_6_0: u32 = 15736;
        // D s_6_1: read-reg s_6_0:u64
        let s_6_1: u64 = {
            let value = state.read_register::<u64>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #0s : i
        let s_6_2: i128 = 0;
        // C s_6_3: const #64s : i
        let s_6_3: i128 = 64;
        // D s_6_4: cast zx s_6_1 -> bv
        let s_6_4: Bits = Bits::new(s_6_1 as u128, 64u16);
        // D s_6_5: bit-extract s_6_4 s_6_2 s_6_3
        let s_6_5: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_3).unwrap(),
        ));
        // D s_6_6: write-var result <= s_6_5
        fn_state.result = s_6_5;
        // N s_6_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var ga#9121:u8
        let s_7_0: u8 = fn_state.ga_9121;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #424u : u32
        let s_7_2: u32 = 424;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // D s_7_6: not s_7_5
        let s_7_6: bool = !s_7_5;
        // N s_7_7: branch s_7_6 b9 b8
        if s_7_6 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_8_0: const #20304u : u32
        let s_8_0: u32 = 20304;
        // D s_8_1: read-reg s_8_0:u64
        let s_8_1: u64 = {
            let value = state.read_register::<u64>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #0s : i
        let s_8_2: i128 = 0;
        // C s_8_3: const #64s : i
        let s_8_3: i128 = 64;
        // D s_8_4: cast zx s_8_1 -> bv
        let s_8_4: Bits = Bits::new(s_8_1 as u128, 64u16);
        // D s_8_5: bit-extract s_8_4 s_8_2 s_8_3
        let s_8_5: Bits = (Bits::new(
            ((s_8_4) >> (s_8_2)).value(),
            u16::try_from(s_8_3).unwrap(),
        ));
        // D s_8_6: write-var result <= s_8_5
        fn_state.result = s_8_5;
        // N s_8_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call Unreachable(s_9_0)
        let s_9_1: () = Unreachable(state, tracer, s_9_0);
        // N s_9_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_10_0: const #32s : i
        let s_10_0: i128 = 32;
        // D s_10_1: read-var N:i
        let s_10_1: i128 = fn_state.N;
        // D s_10_2: cmp-eq s_10_1 s_10_0
        let s_10_2: bool = ((s_10_1) == (s_10_0));
        // N s_10_3: assert s_10_2
        let s_10_3: () = assert!(s_10_2);
        // C s_10_4: const #16983u : u32
        let s_10_4: u32 = 16983;
        // D s_10_5: read-reg s_10_4:u8
        let s_10_5: u8 = {
            let value = state.read_register::<u8>(s_10_4 as isize);
            tracer.read_register(s_10_4 as isize, value);
            value
        };
        // D s_10_6: write-var ga#9110 <= s_10_5
        fn_state.ga_9110 = s_10_5;
        // D s_10_7: read-var ga#9110:u8
        let s_10_7: u8 = fn_state.ga_9110;
        // D s_10_8: cast zx s_10_7 -> bv
        let s_10_8: Bits = Bits::new(s_10_7 as u128, 5u16);
        // C s_10_9: const #360u : u32
        let s_10_9: u32 = 360;
        // D s_10_10: read-reg s_10_9:u8
        let s_10_10: u8 = {
            let value = state.read_register::<u8>(s_10_9 as isize);
            tracer.read_register(s_10_9 as isize, value);
            value
        };
        // D s_10_11: cast zx s_10_10 -> bv
        let s_10_11: Bits = Bits::new(s_10_10 as u128, 5u16);
        // D s_10_12: cmp-eq s_10_8 s_10_11
        let s_10_12: bool = ((s_10_8) == (s_10_11));
        // D s_10_13: not s_10_12
        let s_10_13: bool = !s_10_12;
        // N s_10_14: branch s_10_13 b13 b11
        if s_10_13 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_11_0: const #15536u : u32
        let s_11_0: u32 = 15536;
        // D s_11_1: read-reg s_11_0:u64
        let s_11_1: u64 = {
            let value = state.read_register::<u64>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // C s_11_2: const #0s : i
        let s_11_2: i128 = 0;
        // C s_11_3: const #32s : i
        let s_11_3: i128 = 32;
        // D s_11_4: cast zx s_11_1 -> bv
        let s_11_4: Bits = Bits::new(s_11_1 as u128, 64u16);
        // D s_11_5: bit-extract s_11_4 s_11_2 s_11_3
        let s_11_5: Bits = (Bits::new(
            ((s_11_4) >> (s_11_2)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_6: write-var result <= s_11_5
        fn_state.result = s_11_5;
        // N s_11_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_12_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_13_0: read-var ga#9110:u8
        let s_13_0: u8 = fn_state.ga_9110;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 5u16);
        // C s_13_2: const #368u : u32
        let s_13_2: u32 = 368;
        // D s_13_3: read-reg s_13_2:u8
        let s_13_3: u8 = {
            let value = state.read_register::<u8>(s_13_2 as isize);
            tracer.read_register(s_13_2 as isize, value);
            value
        };
        // D s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 5u16);
        // D s_13_5: cmp-eq s_13_1 s_13_4
        let s_13_5: bool = ((s_13_1) == (s_13_4));
        // D s_13_6: not s_13_5
        let s_13_6: bool = !s_13_5;
        // N s_13_7: branch s_13_6 b15 b14
        if s_13_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_14_0: const #91016u : u32
        let s_14_0: u32 = 91016;
        // D s_14_1: read-reg s_14_0:u64
        let s_14_1: u64 = {
            let value = state.read_register::<u64>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #0s : i
        let s_14_2: i128 = 0;
        // C s_14_3: const #32s : i
        let s_14_3: i128 = 32;
        // D s_14_4: cast zx s_14_1 -> bv
        let s_14_4: Bits = Bits::new(s_14_1 as u128, 64u16);
        // D s_14_5: bit-extract s_14_4 s_14_2 s_14_3
        let s_14_5: Bits = (Bits::new(
            ((s_14_4) >> (s_14_2)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_6: write-var result <= s_14_5
        fn_state.result = s_14_5;
        // N s_14_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_15_0: read-var ga#9110:u8
        let s_15_0: u8 = fn_state.ga_9110;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 5u16);
        // C s_15_2: const #376u : u32
        let s_15_2: u32 = 376;
        // D s_15_3: read-reg s_15_2:u8
        let s_15_3: u8 = {
            let value = state.read_register::<u8>(s_15_2 as isize);
            tracer.read_register(s_15_2 as isize, value);
            value
        };
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 5u16);
        // D s_15_5: cmp-eq s_15_1 s_15_4
        let s_15_5: bool = ((s_15_1) == (s_15_4));
        // D s_15_6: not s_15_5
        let s_15_6: bool = !s_15_5;
        // N s_15_7: branch s_15_6 b17 b16
        if s_15_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call SPSR_svc_read(s_16_0)
        let s_16_1: ProductType700c18a878c5601b = SPSR_svc_read(state, tracer, s_16_0);
        // D s_16_2: write-var ga#9113 <= s_16_1
        fn_state.ga_9113 = s_16_1;
        // D s_16_3: read-var ga#9113.0:struct
        let s_16_3: u32 = fn_state.ga_9113._0;
        // C s_16_4: const #0s : i
        let s_16_4: i128 = 0;
        // C s_16_5: const #32s : i
        let s_16_5: i128 = 32;
        // D s_16_6: cast zx s_16_3 -> bv
        let s_16_6: Bits = Bits::new(s_16_3 as u128, 32u16);
        // D s_16_7: bit-extract s_16_6 s_16_4 s_16_5
        let s_16_7: Bits = (Bits::new(
            ((s_16_6) >> (s_16_4)).value(),
            u16::try_from(s_16_5).unwrap(),
        ));
        // D s_16_8: write-var result <= s_16_7
        fn_state.result = s_16_7;
        // N s_16_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_17_0: read-var ga#9110:u8
        let s_17_0: u8 = fn_state.ga_9110;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 5u16);
        // C s_17_2: const #384u : u32
        let s_17_2: u32 = 384;
        // D s_17_3: read-reg s_17_2:u8
        let s_17_3: u8 = {
            let value = state.read_register::<u8>(s_17_2 as isize);
            tracer.read_register(s_17_2 as isize, value);
            value
        };
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 5u16);
        // D s_17_5: cmp-eq s_17_1 s_17_4
        let s_17_5: bool = ((s_17_1) == (s_17_4));
        // D s_17_6: not s_17_5
        let s_17_6: bool = !s_17_5;
        // N s_17_7: branch s_17_6 b19 b18
        if s_17_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_18_0: const #21136u : u32
        let s_18_0: u32 = 21136;
        // D s_18_1: read-reg s_18_0:u32
        let s_18_1: u32 = {
            let value = state.read_register::<u32>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // C s_18_2: const #0s : i
        let s_18_2: i128 = 0;
        // C s_18_3: const #32s : i
        let s_18_3: i128 = 32;
        // D s_18_4: cast zx s_18_1 -> bv
        let s_18_4: Bits = Bits::new(s_18_1 as u128, 32u16);
        // D s_18_5: bit-extract s_18_4 s_18_2 s_18_3
        let s_18_5: Bits = (Bits::new(
            ((s_18_4) >> (s_18_2)).value(),
            u16::try_from(s_18_3).unwrap(),
        ));
        // D s_18_6: write-var result <= s_18_5
        fn_state.result = s_18_5;
        // N s_18_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_19_0: read-var ga#9110:u8
        let s_19_0: u8 = fn_state.ga_9110;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 5u16);
        // C s_19_2: const #392u : u32
        let s_19_2: u32 = 392;
        // D s_19_3: read-reg s_19_2:u8
        let s_19_3: u8 = {
            let value = state.read_register::<u8>(s_19_2 as isize);
            tracer.read_register(s_19_2 as isize, value);
            value
        };
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 5u16);
        // D s_19_5: cmp-eq s_19_1 s_19_4
        let s_19_5: bool = ((s_19_1) == (s_19_4));
        // D s_19_6: not s_19_5
        let s_19_6: bool = !s_19_5;
        // N s_19_7: branch s_19_6 b21 b20
        if s_19_6 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_20_0: const #20032u : u32
        let s_20_0: u32 = 20032;
        // D s_20_1: read-reg s_20_0:u64
        let s_20_1: u64 = {
            let value = state.read_register::<u64>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // C s_20_2: const #0s : i
        let s_20_2: i128 = 0;
        // C s_20_3: const #32s : i
        let s_20_3: i128 = 32;
        // D s_20_4: cast zx s_20_1 -> bv
        let s_20_4: Bits = Bits::new(s_20_1 as u128, 64u16);
        // D s_20_5: bit-extract s_20_4 s_20_2 s_20_3
        let s_20_5: Bits = (Bits::new(
            ((s_20_4) >> (s_20_2)).value(),
            u16::try_from(s_20_3).unwrap(),
        ));
        // D s_20_6: write-var result <= s_20_5
        fn_state.result = s_20_5;
        // N s_20_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_21_0: read-var ga#9110:u8
        let s_21_0: u8 = fn_state.ga_9110;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 5u16);
        // C s_21_2: const #400u : u32
        let s_21_2: u32 = 400;
        // D s_21_3: read-reg s_21_2:u8
        let s_21_3: u8 = {
            let value = state.read_register::<u8>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 5u16);
        // D s_21_5: cmp-eq s_21_1 s_21_4
        let s_21_5: bool = ((s_21_1) == (s_21_4));
        // D s_21_6: not s_21_5
        let s_21_6: bool = !s_21_5;
        // N s_21_7: branch s_21_6 b23 b22
        if s_21_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call SPSR_hyp_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = SPSR_hyp_read(state, tracer, s_22_0);
        // D s_22_2: write-var ga#9117 <= s_22_1
        fn_state.ga_9117 = s_22_1;
        // D s_22_3: read-var ga#9117.0:struct
        let s_22_3: u32 = fn_state.ga_9117._0;
        // C s_22_4: const #0s : i
        let s_22_4: i128 = 0;
        // C s_22_5: const #32s : i
        let s_22_5: i128 = 32;
        // D s_22_6: cast zx s_22_3 -> bv
        let s_22_6: Bits = Bits::new(s_22_3 as u128, 32u16);
        // D s_22_7: bit-extract s_22_6 s_22_4 s_22_5
        let s_22_7: Bits = (Bits::new(
            ((s_22_6) >> (s_22_4)).value(),
            u16::try_from(s_22_5).unwrap(),
        ));
        // D s_22_8: write-var result <= s_22_7
        fn_state.result = s_22_7;
        // N s_22_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_23_0: read-var ga#9110:u8
        let s_23_0: u8 = fn_state.ga_9110;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 5u16);
        // C s_23_2: const #408u : u32
        let s_23_2: u32 = 408;
        // D s_23_3: read-reg s_23_2:u8
        let s_23_3: u8 = {
            let value = state.read_register::<u8>(s_23_2 as isize);
            tracer.read_register(s_23_2 as isize, value);
            value
        };
        // D s_23_4: cast zx s_23_3 -> bv
        let s_23_4: Bits = Bits::new(s_23_3 as u128, 5u16);
        // D s_23_5: cmp-eq s_23_1 s_23_4
        let s_23_5: bool = ((s_23_1) == (s_23_4));
        // D s_23_6: not s_23_5
        let s_23_6: bool = !s_23_5;
        // N s_23_7: branch s_23_6 b25 b24
        if s_23_6 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_24_0: const #18424u : u32
        let s_24_0: u32 = 18424;
        // D s_24_1: read-reg s_24_0:u64
        let s_24_1: u64 = {
            let value = state.read_register::<u64>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // C s_24_2: const #0s : i
        let s_24_2: i128 = 0;
        // C s_24_3: const #32s : i
        let s_24_3: i128 = 32;
        // D s_24_4: cast zx s_24_1 -> bv
        let s_24_4: Bits = Bits::new(s_24_1 as u128, 64u16);
        // D s_24_5: bit-extract s_24_4 s_24_2 s_24_3
        let s_24_5: Bits = (Bits::new(
            ((s_24_4) >> (s_24_2)).value(),
            u16::try_from(s_24_3).unwrap(),
        ));
        // D s_24_6: write-var result <= s_24_5
        fn_state.result = s_24_5;
        // N s_24_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call Unreachable(s_25_0)
        let s_25_1: () = Unreachable(state, tracer, s_25_0);
        // N s_25_2: jump b12
        return block_12(state, tracer, fn_state);
    }
}
