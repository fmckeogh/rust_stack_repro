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
use u_get_NSACR_Type_cp10::*;
use IsCurrentSecurityState::*;
use Mk_HCPTR_Type::*;
use ELUsingAArch32::*;
use common::*;
pub fn u__get_HCPTR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        gs_37792: bool,
        gs_37791: bool,
        gs_37790: bool,
        tmp: ProductType700c18a878c5601b,
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
    ) -> ProductType700c18a878c5601b {
        // D s_0_0: read-var value_name:struct
        let s_0_0: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_1: write-var tmp <= s_0_0
        fn_state.tmp = s_0_0;
        // D s_0_2: read-var tmp.0:struct
        let s_0_2: u32 = fn_state.tmp._0;
        // C s_0_3: const #32s : i
        let s_0_3: i128 = 32;
        // C s_0_4: const #214712320u : u28
        let s_0_4: u32 = 214712320;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 28u16);
        // D s_0_6: bits-cast zx s_0_5 -> bv length s_0_3
        let s_0_6: Bits = s_0_5.zero_extend(s_0_3);
        // D s_0_7: cast reint s_0_6 -> u32
        let s_0_7: u32 = (s_0_6.value() as u32);
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 32u16);
        // D s_0_9: not s_0_8
        let s_0_9: Bits = !s_0_8;
        // D s_0_10: cast reint s_0_9 -> u32
        let s_0_10: u32 = (s_0_9.value() as u32);
        // D s_0_11: cast zx s_0_2 -> bv
        let s_0_11: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_12: cast zx s_0_10 -> bv
        let s_0_12: Bits = Bits::new(s_0_10 as u128, 32u16);
        // D s_0_13: and s_0_11 s_0_12
        let s_0_13: Bits = ((s_0_11) & (s_0_12));
        // D s_0_14: cast reint s_0_13 -> u32
        let s_0_14: u32 = (s_0_13.value() as u32);
        // D s_0_15: call Mk_HCPTR_Type(s_0_14)
        let s_0_15: ProductType700c18a878c5601b = Mk_HCPTR_Type(state, tracer, s_0_14);
        // D s_0_16: write-var tmp <= s_0_15
        fn_state.tmp = s_0_15;
        // C s_0_17: const #424u : u32
        let s_0_17: u32 = 424;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: u8 = {
            let value = state.read_register::<u8>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // C s_0_19: const #2u : u8
        let s_0_19: u8 = 2;
        // D s_0_20: cmp-lt s_0_18 s_0_19
        let s_0_20: bool = ((s_0_18) < (s_0_19));
        // N s_0_21: branch s_0_20 b12 b1
        if s_0_20 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#37790 <= s_1_0
        fn_state.gs_37790 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_2_0: read-var gs#37790:u8
        let s_2_0: bool = fn_state.gs_37790;
        // N s_2_1: branch s_2_0 b11 b3
        if s_2_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#37791 <= s_3_0
        fn_state.gs_37791 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_4_0: read-var gs#37791:u8
        let s_4_0: bool = fn_state.gs_37791;
        // N s_4_1: branch s_4_0 b10 b5
        if s_4_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#37792 <= s_5_0
        fn_state.gs_37792 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_6_0: read-var gs#37792:u8
        let s_6_0: bool = fn_state.gs_37792;
        // N s_6_1: branch s_6_0 b9 b7
        if s_6_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_8_0: read-var tmp:struct
        let s_8_0: ProductType700c18a878c5601b = fn_state.tmp;
        // N s_8_1: return s_8_0
        return s_8_0;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_9_0: read-var tmp.0:struct
        let s_9_0: u32 = fn_state.tmp._0;
        // C s_9_1: const #32s : i
        let s_9_1: i128 = 32;
        // C s_9_2: const #0u : u8
        let s_9_2: u8 = 0;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 4u16);
        // D s_9_4: bits-cast zx s_9_3 -> bv length s_9_1
        let s_9_4: Bits = s_9_3.zero_extend(s_9_1);
        // D s_9_5: cast reint s_9_4 -> u32
        let s_9_5: u32 = (s_9_4.value() as u32);
        // D s_9_6: cast zx s_9_5 -> bv
        let s_9_6: Bits = Bits::new(s_9_5 as u128, 32u16);
        // D s_9_7: not s_9_6
        let s_9_7: Bits = !s_9_6;
        // D s_9_8: cast reint s_9_7 -> u32
        let s_9_8: u32 = (s_9_7.value() as u32);
        // D s_9_9: cast zx s_9_0 -> bv
        let s_9_9: Bits = Bits::new(s_9_0 as u128, 32u16);
        // D s_9_10: cast zx s_9_8 -> bv
        let s_9_10: Bits = Bits::new(s_9_8 as u128, 32u16);
        // D s_9_11: and s_9_9 s_9_10
        let s_9_11: Bits = ((s_9_9) & (s_9_10));
        // D s_9_12: cast reint s_9_11 -> u32
        let s_9_12: u32 = (s_9_11.value() as u32);
        // C s_9_13: const #32s : i
        let s_9_13: i128 = 32;
        // C s_9_14: const #3072u : u12
        let s_9_14: u16 = 3072;
        // C s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 12u16);
        // D s_9_16: bits-cast zx s_9_15 -> bv length s_9_13
        let s_9_16: Bits = s_9_15.zero_extend(s_9_13);
        // D s_9_17: cast reint s_9_16 -> u32
        let s_9_17: u32 = (s_9_16.value() as u32);
        // D s_9_18: cast zx s_9_12 -> bv
        let s_9_18: Bits = Bits::new(s_9_12 as u128, 32u16);
        // D s_9_19: cast zx s_9_17 -> bv
        let s_9_19: Bits = Bits::new(s_9_17 as u128, 32u16);
        // D s_9_20: or s_9_18 s_9_19
        let s_9_20: Bits = ((s_9_18) | (s_9_19));
        // D s_9_21: cast reint s_9_20 -> u32
        let s_9_21: u32 = (s_9_20.value() as u32);
        // D s_9_22: call Mk_HCPTR_Type(s_9_21)
        let s_9_22: ProductType700c18a878c5601b = Mk_HCPTR_Type(state, tracer, s_9_21);
        // D s_9_23: write-var tmp <= s_9_22
        fn_state.tmp = s_9_22;
        // N s_9_24: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_10_0: const #102488u : u32
        let s_10_0: u32 = 102488;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_NSACR_Type_cp10(s_10_1)
        let s_10_2: bool = u_get_NSACR_Type_cp10(state, tracer, s_10_1);
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // C s_10_4: const #0u : u8
        let s_10_4: bool = false;
        // C s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 1u16);
        // D s_10_6: cmp-eq s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) == (s_10_5));
        // D s_10_7: write-var gs#37792 <= s_10_6
        fn_state.gs_37792 = s_10_6;
        // N s_10_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_11_0: const #3u : u32
        let s_11_0: u32 = 3;
        // S s_11_1: call IsCurrentSecurityState(s_11_0)
        let s_11_1: bool = IsCurrentSecurityState(state, tracer, s_11_0);
        // S s_11_2: not s_11_1
        let s_11_2: bool = !s_11_1;
        // D s_11_3: write-var gs#37791 <= s_11_2
        fn_state.gs_37791 = s_11_2;
        // N s_11_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_12_0: const #424u : u32
        let s_12_0: u32 = 424;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call ELUsingAArch32(s_12_1)
        let s_12_2: bool = ELUsingAArch32(state, tracer, s_12_1);
        // D s_12_3: write-var gs#37790 <= s_12_2
        fn_state.gs_37790 = s_12_2;
        // N s_12_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
