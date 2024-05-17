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
use Unreachable::*;
use SPSR_hyp_read::*;
use SPSR_svc_write::*;
use SPSR_hyp_write::*;
use UsingAArch32::*;
use SPSR_svc_read::*;
use Mk_SPSR_svc_Type::*;
use Mk_SPSR_hyp_Type::*;
use common::*;
pub fn SPSR_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    N: i128,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_4001: u8,
        ga_3976: ProductType700c18a878c5601b,
        ga_3967: u8,
        ga_3990: ProductType700c18a878c5601b,
        N: i128,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        N,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b8 b1
        if s_0_1 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // D s_1_6: write-var ga#4001 <= s_1_5
        fn_state.ga_4001 = s_1_5;
        // D s_1_7: read-var ga#4001:u8
        let s_1_7: u8 = fn_state.ga_4001;
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
        // N s_1_14: branch s_1_13 b3 b2
        if s_1_13 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #90648u : u32
        let s_2_0: u32 = 90648;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // C s_2_2: const #90648u : u32
        let s_2_2: u32 = 90648;
        // N s_2_3: write-reg s_2_2 <= s_2_1
        let s_2_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_2_2 as isize, s_2_1);
            tracer.write_register(s_2_2 as isize, s_2_1);
        };
        // N s_2_4: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#4001:u8
        let s_3_0: u8 = fn_state.ga_4001;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // D s_3_6: not s_3_5
        let s_3_6: bool = !s_3_5;
        // N s_3_7: branch s_3_6 b5 b4
        if s_3_6 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #15736u : u32
        let s_4_0: u32 = 15736;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // C s_4_2: const #15736u : u32
        let s_4_2: u32 = 15736;
        // N s_4_3: write-reg s_4_2 <= s_4_1
        let s_4_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_4_2 as isize, s_4_1);
            tracer.write_register(s_4_2 as isize, s_4_1);
        };
        // N s_4_4: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#4001:u8
        let s_5_0: u8 = fn_state.ga_4001;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #424u : u32
        let s_5_2: u32 = 424;
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
    ) -> () {
        // C s_6_0: const #20304u : u32
        let s_6_0: u32 = 20304;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #20304u : u32
        let s_6_2: u32 = 20304;
        // N s_6_3: write-reg s_6_2 <= s_6_1
        let s_6_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_2 as isize, s_6_1);
            tracer.write_register(s_6_2 as isize, s_6_1);
        };
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call Unreachable(s_7_0)
        let s_7_1: () = Unreachable(state, tracer, s_7_0);
        // N s_7_2: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #32s : i
        let s_8_0: i128 = 32;
        // D s_8_1: read-var N:i
        let s_8_1: i128 = fn_state.N;
        // D s_8_2: cmp-eq s_8_1 s_8_0
        let s_8_2: bool = ((s_8_1) == (s_8_0));
        // N s_8_3: assert s_8_2
        let s_8_3: () = assert!(s_8_2);
        // C s_8_4: const #16983u : u32
        let s_8_4: u32 = 16983;
        // D s_8_5: read-reg s_8_4:u8
        let s_8_5: u8 = {
            let value = state.read_register::<u8>(s_8_4 as isize);
            tracer.read_register(s_8_4 as isize, value);
            value
        };
        // D s_8_6: write-var ga#3967 <= s_8_5
        fn_state.ga_3967 = s_8_5;
        // D s_8_7: read-var ga#3967:u8
        let s_8_7: u8 = fn_state.ga_3967;
        // D s_8_8: cast zx s_8_7 -> bv
        let s_8_8: Bits = Bits::new(s_8_7 as u128, 5u16);
        // C s_8_9: const #360u : u32
        let s_8_9: u32 = 360;
        // D s_8_10: read-reg s_8_9:u8
        let s_8_10: u8 = {
            let value = state.read_register::<u8>(s_8_9 as isize);
            tracer.read_register(s_8_9 as isize, value);
            value
        };
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 5u16);
        // D s_8_12: cmp-eq s_8_8 s_8_11
        let s_8_12: bool = ((s_8_8) == (s_8_11));
        // D s_8_13: not s_8_12
        let s_8_13: bool = !s_8_12;
        // N s_8_14: branch s_8_13 b10 b9
        if s_8_13 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #15536u : u32
        let s_9_0: u32 = 15536;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // C s_9_2: const #15536u : u32
        let s_9_2: u32 = 15536;
        // N s_9_3: write-reg s_9_2 <= s_9_1
        let s_9_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_9_2 as isize, s_9_1);
            tracer.write_register(s_9_2 as isize, s_9_1);
        };
        // N s_9_4: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var ga#3967:u8
        let s_10_0: u8 = fn_state.ga_3967;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 5u16);
        // C s_10_2: const #368u : u32
        let s_10_2: u32 = 368;
        // D s_10_3: read-reg s_10_2:u8
        let s_10_3: u8 = {
            let value = state.read_register::<u8>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 5u16);
        // D s_10_5: cmp-eq s_10_1 s_10_4
        let s_10_5: bool = ((s_10_1) == (s_10_4));
        // D s_10_6: not s_10_5
        let s_10_6: bool = !s_10_5;
        // N s_10_7: branch s_10_6 b12 b11
        if s_10_6 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #91016u : u32
        let s_11_0: u32 = 91016;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // C s_11_2: const #91016u : u32
        let s_11_2: u32 = 91016;
        // N s_11_3: write-reg s_11_2 <= s_11_1
        let s_11_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_11_2 as isize, s_11_1);
            tracer.write_register(s_11_2 as isize, s_11_1);
        };
        // N s_11_4: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var ga#3967:u8
        let s_12_0: u8 = fn_state.ga_3967;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 5u16);
        // C s_12_2: const #376u : u32
        let s_12_2: u32 = 376;
        // D s_12_3: read-reg s_12_2:u8
        let s_12_3: u8 = {
            let value = state.read_register::<u8>(s_12_2 as isize);
            tracer.read_register(s_12_2 as isize, value);
            value
        };
        // D s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 5u16);
        // D s_12_5: cmp-eq s_12_1 s_12_4
        let s_12_5: bool = ((s_12_1) == (s_12_4));
        // D s_12_6: not s_12_5
        let s_12_6: bool = !s_12_5;
        // N s_12_7: branch s_12_6 b14 b13
        if s_12_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call SPSR_svc_read(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = SPSR_svc_read(state, tracer, s_13_0);
        // D s_13_2: write-var ga#3976 <= s_13_1
        fn_state.ga_3976 = s_13_1;
        // D s_13_3: read-var ga#3976.0:struct
        let s_13_3: u32 = fn_state.ga_3976._0;
        // C s_13_4: const #1s : i
        let s_13_4: i128 = 1;
        // D s_13_5: read-var N:i
        let s_13_5: i128 = fn_state.N;
        // D s_13_6: sub s_13_5 s_13_4
        let s_13_6: i128 = ((s_13_5) - (s_13_4));
        // D s_13_7: cast reint s_13_6 -> i64
        let s_13_7: i64 = (s_13_6 as i64);
        // C s_13_8: const #0s : i
        let s_13_8: i128 = 0;
        // C s_13_9: const #32s : i
        let s_13_9: i128 = 32;
        // D s_13_10: read-var value_name:bv
        let s_13_10: Bits = fn_state.value_name;
        // D s_13_11: bit-extract s_13_10 s_13_8 s_13_9
        let s_13_11: Bits = (Bits::new(
            ((s_13_10) >> (s_13_8)).value(),
            u16::try_from(s_13_9).unwrap(),
        ));
        // D s_13_12: cast reint s_13_11 -> u32
        let s_13_12: u32 = (s_13_11.value() as u32);
        // C s_13_13: const #0s : i
        let s_13_13: i128 = 0;
        // D s_13_14: cast zx s_13_3 -> bv
        let s_13_14: Bits = Bits::new(s_13_3 as u128, 32u16);
        // D s_13_15: cast zx s_13_7 -> i
        let s_13_15: i128 = (i128::try_from(s_13_7).unwrap());
        // D s_13_16: cast zx s_13_12 -> bv
        let s_13_16: Bits = Bits::new(s_13_12 as u128, 32u16);
        // D s_13_17: sub s_13_15 s_13_13
        let s_13_17: i128 = ((s_13_15) - (s_13_13));
        // C s_13_18: const #1u : u64
        let s_13_18: u64 = 1;
        // C s_13_19: cast zx s_13_18 -> bv
        let s_13_19: Bits = Bits::new(s_13_18 as u128, 64u16);
        // D s_13_20: lsl s_13_19 s_13_17
        let s_13_20: Bits = s_13_19 << s_13_17;
        // D s_13_21: sub s_13_20 s_13_19
        let s_13_21: Bits = ((s_13_20) - (s_13_19));
        // D s_13_22: and s_13_16 s_13_21
        let s_13_22: Bits = ((s_13_16) & (s_13_21));
        // D s_13_23: lsl s_13_22 s_13_13
        let s_13_23: Bits = s_13_22 << s_13_13;
        // D s_13_24: lsl s_13_21 s_13_13
        let s_13_24: Bits = s_13_21 << s_13_13;
        // D s_13_25: cmpl s_13_24
        let s_13_25: Bits = !s_13_24;
        // D s_13_26: and s_13_14 s_13_25
        let s_13_26: Bits = ((s_13_14) & (s_13_25));
        // D s_13_27: or s_13_26 s_13_23
        let s_13_27: Bits = ((s_13_26) | (s_13_23));
        // D s_13_28: cast reint s_13_27 -> u32
        let s_13_28: u32 = (s_13_27.value() as u32);
        // D s_13_29: call Mk_SPSR_svc_Type(s_13_28)
        let s_13_29: ProductType700c18a878c5601b = Mk_SPSR_svc_Type(
            state,
            tracer,
            s_13_28,
        );
        // D s_13_30: call SPSR_svc_write(s_13_29)
        let s_13_30: () = SPSR_svc_write(state, tracer, s_13_29);
        // N s_13_31: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var ga#3967:u8
        let s_14_0: u8 = fn_state.ga_3967;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 5u16);
        // C s_14_2: const #384u : u32
        let s_14_2: u32 = 384;
        // D s_14_3: read-reg s_14_2:u8
        let s_14_3: u8 = {
            let value = state.read_register::<u8>(s_14_2 as isize);
            tracer.read_register(s_14_2 as isize, value);
            value
        };
        // D s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 5u16);
        // D s_14_5: cmp-eq s_14_1 s_14_4
        let s_14_5: bool = ((s_14_1) == (s_14_4));
        // D s_14_6: not s_14_5
        let s_14_6: bool = !s_14_5;
        // N s_14_7: branch s_14_6 b16 b15
        if s_14_6 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #21136u : u32
        let s_15_0: u32 = 21136;
        // D s_15_1: read-reg s_15_0:struct
        let s_15_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #21136u : u32
        let s_15_2: u32 = 21136;
        // N s_15_3: write-reg s_15_2 <= s_15_1
        let s_15_3: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_15_2 as isize, s_15_1);
            tracer.write_register(s_15_2 as isize, s_15_1);
        };
        // N s_15_4: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var ga#3967:u8
        let s_16_0: u8 = fn_state.ga_3967;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 5u16);
        // C s_16_2: const #392u : u32
        let s_16_2: u32 = 392;
        // D s_16_3: read-reg s_16_2:u8
        let s_16_3: u8 = {
            let value = state.read_register::<u8>(s_16_2 as isize);
            tracer.read_register(s_16_2 as isize, value);
            value
        };
        // D s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 5u16);
        // D s_16_5: cmp-eq s_16_1 s_16_4
        let s_16_5: bool = ((s_16_1) == (s_16_4));
        // D s_16_6: not s_16_5
        let s_16_6: bool = !s_16_5;
        // N s_16_7: branch s_16_6 b18 b17
        if s_16_6 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #20032u : u32
        let s_17_0: u32 = 20032;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #20032u : u32
        let s_17_2: u32 = 20032;
        // N s_17_3: write-reg s_17_2 <= s_17_1
        let s_17_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_17_2 as isize, s_17_1);
            tracer.write_register(s_17_2 as isize, s_17_1);
        };
        // N s_17_4: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var ga#3967:u8
        let s_18_0: u8 = fn_state.ga_3967;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 5u16);
        // C s_18_2: const #400u : u32
        let s_18_2: u32 = 400;
        // D s_18_3: read-reg s_18_2:u8
        let s_18_3: u8 = {
            let value = state.read_register::<u8>(s_18_2 as isize);
            tracer.read_register(s_18_2 as isize, value);
            value
        };
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 5u16);
        // D s_18_5: cmp-eq s_18_1 s_18_4
        let s_18_5: bool = ((s_18_1) == (s_18_4));
        // D s_18_6: not s_18_5
        let s_18_6: bool = !s_18_5;
        // N s_18_7: branch s_18_6 b20 b19
        if s_18_6 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call SPSR_hyp_read(s_19_0)
        let s_19_1: ProductType700c18a878c5601b = SPSR_hyp_read(state, tracer, s_19_0);
        // D s_19_2: write-var ga#3990 <= s_19_1
        fn_state.ga_3990 = s_19_1;
        // D s_19_3: read-var ga#3990.0:struct
        let s_19_3: u32 = fn_state.ga_3990._0;
        // C s_19_4: const #1s : i
        let s_19_4: i128 = 1;
        // D s_19_5: read-var N:i
        let s_19_5: i128 = fn_state.N;
        // D s_19_6: sub s_19_5 s_19_4
        let s_19_6: i128 = ((s_19_5) - (s_19_4));
        // D s_19_7: cast reint s_19_6 -> i64
        let s_19_7: i64 = (s_19_6 as i64);
        // C s_19_8: const #0s : i
        let s_19_8: i128 = 0;
        // C s_19_9: const #32s : i
        let s_19_9: i128 = 32;
        // D s_19_10: read-var value_name:bv
        let s_19_10: Bits = fn_state.value_name;
        // D s_19_11: bit-extract s_19_10 s_19_8 s_19_9
        let s_19_11: Bits = (Bits::new(
            ((s_19_10) >> (s_19_8)).value(),
            u16::try_from(s_19_9).unwrap(),
        ));
        // D s_19_12: cast reint s_19_11 -> u32
        let s_19_12: u32 = (s_19_11.value() as u32);
        // C s_19_13: const #0s : i
        let s_19_13: i128 = 0;
        // D s_19_14: cast zx s_19_3 -> bv
        let s_19_14: Bits = Bits::new(s_19_3 as u128, 32u16);
        // D s_19_15: cast zx s_19_7 -> i
        let s_19_15: i128 = (i128::try_from(s_19_7).unwrap());
        // D s_19_16: cast zx s_19_12 -> bv
        let s_19_16: Bits = Bits::new(s_19_12 as u128, 32u16);
        // D s_19_17: sub s_19_15 s_19_13
        let s_19_17: i128 = ((s_19_15) - (s_19_13));
        // C s_19_18: const #1u : u64
        let s_19_18: u64 = 1;
        // C s_19_19: cast zx s_19_18 -> bv
        let s_19_19: Bits = Bits::new(s_19_18 as u128, 64u16);
        // D s_19_20: lsl s_19_19 s_19_17
        let s_19_20: Bits = s_19_19 << s_19_17;
        // D s_19_21: sub s_19_20 s_19_19
        let s_19_21: Bits = ((s_19_20) - (s_19_19));
        // D s_19_22: and s_19_16 s_19_21
        let s_19_22: Bits = ((s_19_16) & (s_19_21));
        // D s_19_23: lsl s_19_22 s_19_13
        let s_19_23: Bits = s_19_22 << s_19_13;
        // D s_19_24: lsl s_19_21 s_19_13
        let s_19_24: Bits = s_19_21 << s_19_13;
        // D s_19_25: cmpl s_19_24
        let s_19_25: Bits = !s_19_24;
        // D s_19_26: and s_19_14 s_19_25
        let s_19_26: Bits = ((s_19_14) & (s_19_25));
        // D s_19_27: or s_19_26 s_19_23
        let s_19_27: Bits = ((s_19_26) | (s_19_23));
        // D s_19_28: cast reint s_19_27 -> u32
        let s_19_28: u32 = (s_19_27.value() as u32);
        // D s_19_29: call Mk_SPSR_hyp_Type(s_19_28)
        let s_19_29: ProductType700c18a878c5601b = Mk_SPSR_hyp_Type(
            state,
            tracer,
            s_19_28,
        );
        // D s_19_30: call SPSR_hyp_write(s_19_29)
        let s_19_30: () = SPSR_hyp_write(state, tracer, s_19_29);
        // N s_19_31: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var ga#3967:u8
        let s_20_0: u8 = fn_state.ga_3967;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 5u16);
        // C s_20_2: const #408u : u32
        let s_20_2: u32 = 408;
        // D s_20_3: read-reg s_20_2:u8
        let s_20_3: u8 = {
            let value = state.read_register::<u8>(s_20_2 as isize);
            tracer.read_register(s_20_2 as isize, value);
            value
        };
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 5u16);
        // D s_20_5: cmp-eq s_20_1 s_20_4
        let s_20_5: bool = ((s_20_1) == (s_20_4));
        // D s_20_6: not s_20_5
        let s_20_6: bool = !s_20_5;
        // N s_20_7: branch s_20_6 b22 b21
        if s_20_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #18424u : u32
        let s_21_0: u32 = 18424;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // C s_21_2: const #18424u : u32
        let s_21_2: u32 = 18424;
        // N s_21_3: write-reg s_21_2 <= s_21_1
        let s_21_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_21_2 as isize, s_21_1);
            tracer.write_register(s_21_2 as isize, s_21_1);
        };
        // N s_21_4: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call Unreachable(s_22_0)
        let s_22_1: () = Unreachable(state, tracer, s_22_0);
        // N s_22_2: return
        return;
    }
}
