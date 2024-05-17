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
use sail_mem_read::*;
use CreateAccDescGPR::*;
use read_request::*;
use common::*;
pub fn gic_read_ram<T: Tracer>(state: &mut State, tracer: &T, offset: u16) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_331314: u32,
        ga_370403: SumTypebfdf2f926abd32c5,
        offset: u16,
    }
    let fn_state = FunctionState {
        offset,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // C s_0_1: const #0u : u8
        let s_0_1: bool = false;
        // C s_0_2: const #1u : u8
        let s_0_2: bool = true;
        // C s_0_3: const #0u : u8
        let s_0_3: bool = false;
        // S s_0_4: call CreateAccDescGPR(s_0_0, s_0_1, s_0_2, s_0_3)
        let s_0_4: ProductType9878976b5bcce9c9 = CreateAccDescGPR(
            state,
            tracer,
            s_0_0,
            s_0_1,
            s_0_2,
            s_0_3,
        );
        // C s_0_5: const #1360u : u32
        let s_0_5: u32 = 1360;
        // D s_0_6: read-reg s_0_5:u16
        let s_0_6: u16 = {
            let value = state.read_register::<u16>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 16u16);
        // D s_0_8: read-var offset:u16
        let s_0_8: u16 = fn_state.offset;
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 16u16);
        // D s_0_10: cast reint s_0_7 -> u128
        let s_0_10: u128 = (s_0_7.value() as u128);
        // D s_0_11: size-of s_0_7
        let s_0_11: u16 = s_0_7.length();
        // D s_0_12: cast reint s_0_9 -> u128
        let s_0_12: u128 = (s_0_9.value() as u128);
        // D s_0_13: size-of s_0_9
        let s_0_13: u16 = s_0_9.length();
        // D s_0_14: lsl s_0_10 s_0_13
        let s_0_14: u128 = s_0_10 << s_0_13;
        // D s_0_15: or s_0_14 s_0_12
        let s_0_15: u128 = ((s_0_14) | (s_0_12));
        // D s_0_16: add s_0_11 s_0_13
        let s_0_16: u16 = (s_0_11 + s_0_13);
        // D s_0_17: create-bits s_0_15 s_0_16
        let s_0_17: Bits = Bits::new(s_0_15, s_0_16);
        // D s_0_18: cast reint s_0_17 -> u32
        let s_0_18: u32 = (s_0_17.value() as u32);
        // C s_0_19: const #56s : i
        let s_0_19: i128 = 56;
        // D s_0_20: cast zx s_0_18 -> bv
        let s_0_20: Bits = Bits::new(s_0_18 as u128, 32u16);
        // D s_0_21: bits-cast zx s_0_20 -> bv length s_0_19
        let s_0_21: Bits = s_0_20.zero_extend(s_0_19);
        // D s_0_22: cast reint s_0_21 -> u56
        let s_0_22: u64 = (s_0_21.value() as u64);
        // C s_0_23: const #() : ()
        let s_0_23: () = ();
        // D s_0_24: create-sum enum = 0:"s_0_23"
        let s_0_24: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_0_23);
        // C s_0_25: const #64s : i
        let s_0_25: i128 = 64;
        // D s_0_26: cast zx s_0_22 -> bv
        let s_0_26: Bits = Bits::new(s_0_22 as u128, 56u16);
        // D s_0_27: bits-cast zx s_0_26 -> bv length s_0_25
        let s_0_27: Bits = s_0_26.zero_extend(s_0_25);
        // D s_0_28: cast reint s_0_27 -> u64
        let s_0_28: u64 = (s_0_27.value() as u64);
        // C s_0_29: const #4s : i
        let s_0_29: i128 = 4;
        // D s_0_30: call read_request(s_0_4, s_0_24, s_0_29, s_0_28, s_0_22)
        let s_0_30: ProductTypeee12e330a5f80ce = read_request(
            state,
            tracer,
            s_0_4,
            s_0_24,
            s_0_29,
            s_0_28,
            s_0_22,
        );
        // D s_0_31: call sail_mem_read(s_0_30)
        let s_0_31: SumTypebfdf2f926abd32c5 = sail_mem_read(state, tracer, s_0_30);
        // D s_0_32: write-var ga#370403 <= s_0_31
        fn_state.ga_370403 = s_0_31;
        // D s_0_33: read-var ga#370403:enum
        let s_0_33: SumTypebfdf2f926abd32c5 = fn_state.ga_370403;
        // D s_0_34: matches-sum s_0_33 1
        let s_0_34: bool = matches!(s_0_33, SumTypebfdf2f926abd32c5::_1(_));
        // N s_0_35: branch s_0_34 b3 b1
        if s_0_34 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_1_0: read-var ga#370403:enum
        let s_1_0: SumTypebfdf2f926abd32c5 = fn_state.ga_370403;
        // D s_1_1: unwrap-sum s_1_0 1
        let s_1_1: ProductTypead6b611358cb4242 = match s_1_0 {
            SumTypebfdf2f926abd32c5::_1(inner) => inner,
            _ => panic!("unwrap sum failed"),
        };
        // D s_1_2: extract-field s_1_1.0
        let s_1_2: Bits = s_1_1._0;
        // D s_1_3: cast reint s_1_2 -> u32
        let s_1_3: u32 = (s_1_2.value() as u32);
        // D s_1_4: write-var gs#331314 <= s_1_3
        fn_state.gs_331314 = s_1_3;
        // N s_1_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var gs#331314:u32
        let s_2_0: u32 = fn_state.gs_331314;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_3_0: read-var ga#370403:enum
        let s_3_0: SumTypebfdf2f926abd32c5 = fn_state.ga_370403;
        // D s_3_1: matches-sum s_3_0 0
        let s_3_1: bool = matches!(s_3_0, SumTypebfdf2f926abd32c5::_0(_));
        // N s_3_2: branch s_3_1 b5 b4
        if s_3_1 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_5_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
