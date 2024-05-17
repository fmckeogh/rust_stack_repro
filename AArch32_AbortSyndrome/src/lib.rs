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
use AArch32_FaultSyndrome::*;
use IPAValid::*;
use ExceptionSyndrome::*;
use common::*;
pub fn AArch32_AbortSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    exceptype: u32,
    fault: ProductType1d757adad216cdef,
    vaddress: u32,
    target_el: u8,
) -> ProductTypeb7f99f96751e17c4 {
    #[derive(Default)]
    struct FunctionState {
        ga_6165: ProductTypeda0231e9dc169f81,
        ga_6162: ProductTypeda0231e9dc169f81,
        except: ProductTypeb7f99f96751e17c4,
        exceptype: u32,
        fault: ProductType1d757adad216cdef,
        vaddress: u32,
        target_el: u8,
    }
    let fn_state = FunctionState {
        exceptype,
        fault,
        vaddress,
        target_el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_0_0: read-var exceptype:u32
        let s_0_0: u32 = fn_state.exceptype;
        // D s_0_1: call ExceptionSyndrome(s_0_0)
        let s_0_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_0_0);
        // D s_0_2: write-var except <= s_0_1
        fn_state.except = s_0_1;
        // D s_0_3: read-var exceptype:u32
        let s_0_3: u32 = fn_state.exceptype;
        // C s_0_4: const #19u : u32
        let s_0_4: u32 = 19;
        // D s_0_5: cmp-eq s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) == (s_0_4));
        // D s_0_6: read-var fault:struct
        let s_0_6: ProductType1d757adad216cdef = fn_state.fault;
        // D s_0_7: call AArch32_FaultSyndrome(s_0_5, s_0_6)
        let s_0_7: u32 = AArch32_FaultSyndrome(state, tracer, s_0_5, s_0_6);
        // D s_0_8: write-var except.6 <= s_0_7
        fn_state.except._6 = s_0_7;
        // C s_0_9: const #64s : i
        let s_0_9: i128 = 64;
        // D s_0_10: read-var vaddress:u32
        let s_0_10: u32 = fn_state.vaddress;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 32u16);
        // D s_0_12: bits-cast zx s_0_11 -> bv length s_0_9
        let s_0_12: Bits = s_0_11.zero_extend(s_0_9);
        // D s_0_13: cast reint s_0_12 -> u64
        let s_0_13: u64 = (s_0_12.value() as u64);
        // D s_0_14: write-var except.9 <= s_0_13
        fn_state.except._9 = s_0_13;
        // D s_0_15: read-var fault:struct
        let s_0_15: ProductType1d757adad216cdef = fn_state.fault;
        // D s_0_16: call IPAValid(s_0_15)
        let s_0_16: bool = IPAValid(state, tracer, s_0_15);
        // N s_0_17: branch s_0_16 b3 b1
        if s_0_16 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var except.3 <= s_1_0
        fn_state.except._3 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_2_0: read-var except:struct
        let s_2_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var except.3 <= s_3_0
        fn_state.except._3 = s_3_0;
        // D s_3_2: read-var fault.8:struct
        let s_3_2: ProductTypeda0231e9dc169f81 = fn_state.fault._8;
        // D s_3_3: write-var ga#6162 <= s_3_2
        fn_state.ga_6162 = s_3_2;
        // D s_3_4: read-var ga#6162.1:struct
        let s_3_4: u32 = fn_state.ga_6162._1;
        // C s_3_5: const #0u : u32
        let s_3_5: u32 = 0;
        // D s_3_6: cmp-eq s_3_4 s_3_5
        let s_3_6: bool = ((s_3_4) == (s_3_5));
        // N s_3_7: branch s_3_6 b6 b4
        if s_3_6 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var except.0 <= s_4_0
        fn_state.except._0 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_5_0: read-var fault.8:struct
        let s_5_0: ProductTypeda0231e9dc169f81 = fn_state.fault._8;
        // D s_5_1: write-var ga#6165 <= s_5_0
        fn_state.ga_6165 = s_5_0;
        // D s_5_2: read-var ga#6165.0:struct
        let s_5_2: u64 = fn_state.ga_6165._0;
        // C s_5_3: const #56s : i
        let s_5_3: i128 = 56;
        // D s_5_4: cast zx s_5_2 -> bv
        let s_5_4: Bits = Bits::new(s_5_2 as u128, 56u16);
        // D s_5_5: bits-cast zx s_5_4 -> bv length s_5_3
        let s_5_5: Bits = s_5_4.zero_extend(s_5_3);
        // D s_5_6: cast reint s_5_5 -> u56
        let s_5_6: u64 = (s_5_5.value() as u64);
        // D s_5_7: write-var except.2 <= s_5_6
        fn_state.except._2 = s_5_6;
        // N s_5_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var except.0 <= s_6_0
        fn_state.except._0 = s_6_0;
        // N s_6_2: jump b5
        return block_5(state, tracer, fn_state);
    }
}
