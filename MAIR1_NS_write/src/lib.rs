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
use NMRR_read::*;
use NMRR_NS_write::*;
use NMRR_write::*;
use ELUsingAArch32::*;
use Mk_NMRR_Type::*;
use NMRR_NS_read::*;
use common::*;
pub fn MAIR1_NS_write<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        ga_28571: ProductType700c18a878c5601b,
        gs_37039: bool,
        ga_28585: ProductType700c18a878c5601b,
        value_name: ProductType700c18a878c5601b,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_1: write-var r <= s_0_0
        fn_state.r = s_0_0;
        // C s_0_2: const #424u : u32
        let s_0_2: u32 = 424;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: call ELUsingAArch32(s_0_3)
        let s_0_4: bool = ELUsingAArch32(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b8 b1
        if s_0_4 {
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
        // C s_1_0: const #424u : u32
        let s_1_0: u32 = 424;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #2u : u8
        let s_1_2: u8 = 2;
        // D s_1_3: cmp-lt s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) < (s_1_2));
        // D s_1_4: not s_1_3
        let s_1_4: bool = !s_1_3;
        // N s_1_5: branch s_1_4 b7 b2
        if s_1_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #424u : u32
        let s_2_0: u32 = 424;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call ELUsingAArch32(s_2_1)
        let s_2_2: bool = ELUsingAArch32(state, tracer, s_2_1);
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // D s_2_4: write-var gs#37039 <= s_2_3
        fn_state.gs_37039 = s_2_3;
        // N s_2_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#37039:u8
        let s_3_0: bool = fn_state.gs_37039;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call NMRR_read(s_4_0)
        let s_4_1: ProductType700c18a878c5601b = NMRR_read(state, tracer, s_4_0);
        // D s_4_2: write-var ga#28585 <= s_4_1
        fn_state.ga_28585 = s_4_1;
        // D s_4_3: read-var ga#28585.0:struct
        let s_4_3: u32 = fn_state.ga_28585._0;
        // D s_4_4: read-var r.0:struct
        let s_4_4: u32 = fn_state.r._0;
        // C s_4_5: const #0s : i
        let s_4_5: i128 = 0;
        // C s_4_6: const #32s : i
        let s_4_6: i128 = 32;
        // D s_4_7: cast zx s_4_4 -> bv
        let s_4_7: Bits = Bits::new(s_4_4 as u128, 32u16);
        // D s_4_8: bit-extract s_4_7 s_4_5 s_4_6
        let s_4_8: Bits = (Bits::new(
            ((s_4_7) >> (s_4_5)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_9: cast reint s_4_8 -> u32
        let s_4_9: u32 = (s_4_8.value() as u32);
        // C s_4_10: const #32s : i
        let s_4_10: i128 = 32;
        // C s_4_11: const #0s : i
        let s_4_11: i128 = 0;
        // D s_4_12: cast zx s_4_3 -> bv
        let s_4_12: Bits = Bits::new(s_4_3 as u128, 32u16);
        // D s_4_13: cast zx s_4_9 -> bv
        let s_4_13: Bits = Bits::new(s_4_9 as u128, 32u16);
        // C s_4_14: const #1u : u64
        let s_4_14: u64 = 1;
        // C s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 64u16);
        // C s_4_16: lsl s_4_15 s_4_10
        let s_4_16: Bits = s_4_15 << s_4_10;
        // C s_4_17: sub s_4_16 s_4_15
        let s_4_17: Bits = ((s_4_16) - (s_4_15));
        // D s_4_18: and s_4_13 s_4_17
        let s_4_18: Bits = ((s_4_13) & (s_4_17));
        // D s_4_19: lsl s_4_18 s_4_11
        let s_4_19: Bits = s_4_18 << s_4_11;
        // C s_4_20: lsl s_4_17 s_4_11
        let s_4_20: Bits = s_4_17 << s_4_11;
        // C s_4_21: cmpl s_4_20
        let s_4_21: Bits = !s_4_20;
        // D s_4_22: and s_4_12 s_4_21
        let s_4_22: Bits = ((s_4_12) & (s_4_21));
        // D s_4_23: or s_4_22 s_4_19
        let s_4_23: Bits = ((s_4_22) | (s_4_19));
        // D s_4_24: cast reint s_4_23 -> u32
        let s_4_24: u32 = (s_4_23.value() as u32);
        // D s_4_25: call Mk_NMRR_Type(s_4_24)
        let s_4_25: ProductType700c18a878c5601b = Mk_NMRR_Type(state, tracer, s_4_24);
        // D s_4_26: call NMRR_write(s_4_25)
        let s_4_26: () = NMRR_write(state, tracer, s_4_25);
        // N s_4_27: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var r:struct
        let s_5_0: ProductType700c18a878c5601b = fn_state.r;
        // C s_5_1: const #17208u : u32
        let s_5_1: u32 = 17208;
        // N s_5_2: write-reg s_5_1 <= s_5_0
        let s_5_2: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_5_1 as isize, s_5_0);
            tracer.write_register(s_5_1 as isize, s_5_0);
        };
        // N s_5_3: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #22920u : u32
        let s_6_0: u32 = 22920;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #22920u : u32
        let s_6_2: u32 = 22920;
        // N s_6_3: write-reg s_6_2 <= s_6_1
        let s_6_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_2 as isize, s_6_1);
            tracer.write_register(s_6_2 as isize, s_6_1);
        };
        // N s_6_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#37039 <= s_7_0
        fn_state.gs_37039 = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call NMRR_NS_read(s_8_0)
        let s_8_1: ProductType700c18a878c5601b = NMRR_NS_read(state, tracer, s_8_0);
        // D s_8_2: write-var ga#28571 <= s_8_1
        fn_state.ga_28571 = s_8_1;
        // D s_8_3: read-var ga#28571.0:struct
        let s_8_3: u32 = fn_state.ga_28571._0;
        // D s_8_4: read-var r.0:struct
        let s_8_4: u32 = fn_state.r._0;
        // C s_8_5: const #0s : i
        let s_8_5: i128 = 0;
        // C s_8_6: const #32s : i
        let s_8_6: i128 = 32;
        // D s_8_7: cast zx s_8_4 -> bv
        let s_8_7: Bits = Bits::new(s_8_4 as u128, 32u16);
        // D s_8_8: bit-extract s_8_7 s_8_5 s_8_6
        let s_8_8: Bits = (Bits::new(
            ((s_8_7) >> (s_8_5)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_9: cast reint s_8_8 -> u32
        let s_8_9: u32 = (s_8_8.value() as u32);
        // C s_8_10: const #32s : i
        let s_8_10: i128 = 32;
        // C s_8_11: const #0s : i
        let s_8_11: i128 = 0;
        // D s_8_12: cast zx s_8_3 -> bv
        let s_8_12: Bits = Bits::new(s_8_3 as u128, 32u16);
        // D s_8_13: cast zx s_8_9 -> bv
        let s_8_13: Bits = Bits::new(s_8_9 as u128, 32u16);
        // C s_8_14: const #1u : u64
        let s_8_14: u64 = 1;
        // C s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 64u16);
        // C s_8_16: lsl s_8_15 s_8_10
        let s_8_16: Bits = s_8_15 << s_8_10;
        // C s_8_17: sub s_8_16 s_8_15
        let s_8_17: Bits = ((s_8_16) - (s_8_15));
        // D s_8_18: and s_8_13 s_8_17
        let s_8_18: Bits = ((s_8_13) & (s_8_17));
        // D s_8_19: lsl s_8_18 s_8_11
        let s_8_19: Bits = s_8_18 << s_8_11;
        // C s_8_20: lsl s_8_17 s_8_11
        let s_8_20: Bits = s_8_17 << s_8_11;
        // C s_8_21: cmpl s_8_20
        let s_8_21: Bits = !s_8_20;
        // D s_8_22: and s_8_12 s_8_21
        let s_8_22: Bits = ((s_8_12) & (s_8_21));
        // D s_8_23: or s_8_22 s_8_19
        let s_8_23: Bits = ((s_8_22) | (s_8_19));
        // D s_8_24: cast reint s_8_23 -> u32
        let s_8_24: u32 = (s_8_23.value() as u32);
        // D s_8_25: call Mk_NMRR_Type(s_8_24)
        let s_8_25: ProductType700c18a878c5601b = Mk_NMRR_Type(state, tracer, s_8_24);
        // D s_8_26: call NMRR_NS_write(s_8_25)
        let s_8_26: () = NMRR_NS_write(state, tracer, s_8_25);
        // N s_8_27: jump b5
        return block_5(state, tracer, fn_state);
    }
}
